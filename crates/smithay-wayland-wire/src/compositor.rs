use crate::protocol as proto;
use crate::state as st;
use crate::transport::WaylandTransport;
use crate::wire::{WaylandArgument, WaylandMessage, WireError};
use std::collections::HashMap;
use tracing::{debug, info};

pub struct WaylandGlobal {
    pub name: u32,
    pub interface: &'static str,
    pub version: u32,
}

pub struct WaylandCompositor {
    pub globals: Vec<WaylandGlobal>,
    pub surfaces: st::SurfaceRegistry,
    pub dmabuf_table: st::D3D12SharedMappingTable,
    pub seat: st::WaylandSeat,
    pub outputs: Vec<st::WaylandOutput>,
    pub next_global_name: u32,
    pub client_objects: HashMap<u32, String>,
}

impl WaylandCompositor {
    pub fn new() -> Self {
        let mut compositor = Self {
            globals: Vec::new(),
            surfaces: st::SurfaceRegistry::new(),
            dmabuf_table: st::D3D12SharedMappingTable::new(),
            seat: st::WaylandSeat::new("seat0"),
            outputs: vec![st::WaylandOutput::new_rtx4070_120hz(1)],
            next_global_name: 1,
            client_objects: HashMap::new(),
        };

        compositor.register_global(proto::INTERFACE_WL_COMPOSITOR, 6);
        compositor.register_global(proto::INTERFACE_WL_SUBCOMPOSITOR, 1);
        compositor.register_global(proto::INTERFACE_WL_SHM, 1);
        compositor.register_global(proto::INTERFACE_XDG_WM_BASE, 4);
        compositor.register_global(proto::INTERFACE_ZWP_LINUX_DMABUF_V1, 4);
        compositor.register_global(proto::INTERFACE_WL_SEAT, 8);
        compositor.register_global(proto::INTERFACE_WL_OUTPUT, 4);
        compositor.register_global(proto::INTERFACE_WP_FRACTIONAL_SCALE_MANAGER_V1, 1);
        compositor.register_global(proto::INTERFACE_WP_VIEWPORTER, 1);
        compositor.register_global(proto::INTERFACE_EXT_WORKSPACE_MANAGER_V1, 1);

        compositor
    }

    pub fn register_global(&mut self, interface: &'static str, version: u32) -> u32 {
        let name = self.next_global_name;
        self.next_global_name += 1;
        self.globals.push(WaylandGlobal {
            name,
            interface,
            version,
        });
        name
    }

    pub fn advertise_globals(&self, registry_id: u32, transport: &mut dyn WaylandTransport) -> Result<(), WireError> {
        for g in &self.globals {
            let msg = WaylandMessage::new(
                registry_id,
                proto::registry::EVT_GLOBAL,
                vec![
                    WaylandArgument::Uint(g.name),
                    WaylandArgument::String(g.interface.to_string()),
                    WaylandArgument::Uint(g.version),
                ],
            );
            transport.send(&msg)?;
        }
        Ok(())
    }

    pub fn dispatch(&mut self, msg: WaylandMessage, transport: &mut dyn WaylandTransport) -> Result<(), WireError> {
        debug!("Dispatching message: {}", msg);

        // Object 1 is always wl_display
        if msg.sender_id == 1 {
            match msg.opcode {
                proto::display::REQ_SYNC => {
                    if let Some(callback_id) = msg.args.first().and_then(|a| a.as_uint()) {
                        let done_msg = WaylandMessage::new(
                            callback_id,
                            0, // wl_callback.done
                            vec![WaylandArgument::Uint(0)],
                        );
                        transport.send(&done_msg)?;
                    }
                }
                proto::display::REQ_GET_REGISTRY => {
                    if let Some(registry_id) = msg.args.first().and_then(|a| a.as_uint()) {
                        self.client_objects.insert(registry_id, proto::INTERFACE_WL_REGISTRY.into());
                        self.advertise_globals(registry_id, transport)?;
                    }
                }
                _ => {}
            }
            return Ok(());
        }

        let obj_interface = self.client_objects.get(&msg.sender_id).cloned();
        if let Some(interface) = obj_interface {
            match interface.as_str() {
                proto::INTERFACE_WL_REGISTRY => {
                    if msg.opcode == proto::registry::REQ_BIND {
                        // args: name: uint, id: new_id
                        if msg.args.len() >= 2 {
                            let name = msg.args[0].as_uint().unwrap_or(0);
                            let new_id = msg.args[1].as_uint().unwrap_or(0);
                            if let Some(global) = self.globals.iter().find(|g| g.name == name) {
                                self.client_objects.insert(new_id, global.interface.to_string());
                                info!("Client bound global {} -> id {}", global.interface, new_id);
                            }
                        }
                    }
                }
                proto::INTERFACE_WL_COMPOSITOR => {
                    if msg.opcode == proto::compositor::REQ_CREATE_SURFACE {
                        if let Some(surface_id) = msg.args.first().and_then(|a| a.as_uint()) {
                            self.surfaces.create_surface(surface_id);
                            self.client_objects.insert(surface_id, proto::INTERFACE_WL_SURFACE.into());
                            info!("Created Wayland surface id {}", surface_id);
                        }
                    }
                }
                proto::INTERFACE_WL_SURFACE => {
                    match msg.opcode {
                        proto::surface::REQ_ATTACH => {
                            if msg.args.len() >= 3 {
                                let buffer_id = msg.args[0].as_uint().unwrap_or(0);
                                let x = msg.args[1].as_uint().unwrap_or(0) as i32;
                                let y = msg.args[2].as_uint().unwrap_or(0) as i32;
                                if let Some(surf) = self.surfaces.get_surface_mut(msg.sender_id) {
                                    surf.attach_buffer(buffer_id, x, y);
                                }
                            }
                        }
                        proto::surface::REQ_DAMAGE => {
                            if msg.args.len() >= 4 {
                                let x = msg.args[0].as_uint().unwrap_or(0) as i32;
                                let y = msg.args[1].as_uint().unwrap_or(0) as i32;
                                let width = msg.args[2].as_uint().unwrap_or(0) as i32;
                                let height = msg.args[3].as_uint().unwrap_or(0) as i32;
                                if let Some(surf) = self.surfaces.get_surface_mut(msg.sender_id) {
                                    surf.add_damage(st::Rect { x, y, width, height });
                                }
                            }
                        }
                        proto::surface::REQ_COMMIT => {
                            if let Some(surf) = self.surfaces.get_surface_mut(msg.sender_id) {
                                let frame = surf.commit();
                                debug!("Committed surface {} (frame {})", msg.sender_id, frame);
                            }
                        }
                        _ => {}
                    }
                }
                proto::INTERFACE_XDG_WM_BASE => {
                    if msg.opcode == proto::xdg_wm_base::REQ_GET_XDG_SURFACE {
                        if msg.args.len() >= 2 {
                            let xdg_surface_id = msg.args[0].as_uint().unwrap_or(0);
                            let wl_surface_id = msg.args[1].as_uint().unwrap_or(0);
                            self.client_objects.insert(xdg_surface_id, proto::INTERFACE_XDG_SURFACE.into());
                            if let Some(surf) = self.surfaces.get_surface_mut(wl_surface_id) {
                                surf.role = st::SurfaceRole::XdgToplevel;
                            }
                        }
                    }
                }
                proto::INTERFACE_XDG_SURFACE => {
                    if msg.opcode == proto::xdg_surface::REQ_GET_TOPLEVEL {
                        if let Some(toplevel_id) = msg.args.first().and_then(|a| a.as_uint()) {
                            self.client_objects.insert(toplevel_id, proto::INTERFACE_XDG_TOPLEVEL.into());
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
