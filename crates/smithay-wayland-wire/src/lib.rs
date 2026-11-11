pub mod compositor;
pub mod protocol;
pub mod state;
pub mod transport;
pub mod wire;

pub use compositor::{WaylandCompositor, WaylandGlobal};
pub use protocol as proto;
pub use state as st;
pub use transport::{hyperv::SockAddrHv, LoopbackTransport, WaylandTransport};
pub use wire::{WaylandArgument, WaylandMessage, WireError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wayland_wire_encoding_decoding() {
        let msg = WaylandMessage::new(
            2,
            0, // registry.bind
            vec![
                WaylandArgument::Uint(1),
                WaylandArgument::String("wl_compositor".into()),
                WaylandArgument::Uint(6),
                WaylandArgument::NewId(3),
            ],
        );

        let mut buf = Vec::new();
        msg.encode(&mut buf);
        assert_eq!(buf.len() % 4, 0);

        let (sender, opcode, size, payload) =
            WaylandMessage::decode_raw(&buf).expect("Failed to decode raw wire header");
        assert_eq!(sender, 2);
        assert_eq!(opcode, 0);
        assert_eq!(size, buf.len());
        assert!(!payload.is_empty());
    }

    #[test]
    fn test_compositor_globals_and_dispatch() {
        let mut compositor = WaylandCompositor::new();
        let mut transport = LoopbackTransport::new();

        // Simulate client requesting registry
        let req_registry = WaylandMessage::new(
            1, // wl_display
            proto::display::REQ_GET_REGISTRY,
            vec![WaylandArgument::NewId(2)],
        );
        compositor.dispatch(req_registry, &mut transport).unwrap();

        // Server should advertise all 10 globals to client
        let mut advertised = 0;
        while let Some(msg) = transport.server_to_client.pop_front() {
            assert_eq!(msg.sender_id, 2); // sent to registry id 2
            assert_eq!(msg.opcode, proto::registry::EVT_GLOBAL);
            advertised += 1;
        }
        assert_eq!(advertised, 10);

        // Client binds wl_compositor (global 1 -> id 3)
        let bind_compositor = WaylandMessage::new(
            2,
            proto::registry::REQ_BIND,
            vec![
                WaylandArgument::Uint(1),
                WaylandArgument::NewId(3),
            ],
        );
        compositor.dispatch(bind_compositor, &mut transport).unwrap();

        // Client creates surface (id 4)
        let create_surface = WaylandMessage::new(
            3,
            proto::compositor::REQ_CREATE_SURFACE,
            vec![WaylandArgument::NewId(4)],
        );
        compositor.dispatch(create_surface, &mut transport).unwrap();
        assert!(compositor.surfaces.get_surface(4).is_some());

        // Client attaches buffer and commits
        let attach = WaylandMessage::new(
            4,
            proto::surface::REQ_ATTACH,
            vec![WaylandArgument::Uint(100), WaylandArgument::Uint(0), WaylandArgument::Uint(0)],
        );
        compositor.dispatch(attach, &mut transport).unwrap();

        let commit = WaylandMessage::new(4, proto::surface::REQ_COMMIT, vec![]);
        compositor.dispatch(commit, &mut transport).unwrap();

        let surf = compositor.surfaces.get_surface(4).unwrap();
        assert_eq!(surf.committed_frame_count, 1);
        assert_eq!(surf.current.buffer_id, Some(100));
    }

    #[test]
    fn test_dmabuf_d3d12_shared_mapping() {
        let mut table = st::D3D12SharedMappingTable::new();
        let planes = vec![st::DmaBufPlane {
            fd: 42,
            plane_index: 0,
            offset: 0,
            stride: 1920 * 4,
            modifier: 0,
        }];
        table.register_dmabuf(
            201,
            1920,
            1080,
            0x34325241, // DRM_FORMAT_ARGB8888
            0,
            planes,
            None,
        );

        let entry = table.get(201).expect("Must find imported dmabuf");
        assert_eq!(entry.width, 1920);
        assert_eq!(entry.height, 1080);
        assert_eq!(entry.format_fourcc, 0x34325241);
    }
}

// Transport trait implementation for LoopbackTransport
impl WaylandTransport for LoopbackTransport {
    fn send(&mut self, msg: &WaylandMessage) -> Result<(), WireError> {
        self.server_send(msg.clone());
        Ok(())
    }

    fn receive(&mut self) -> Result<Option<WaylandMessage>, WireError> {
        Ok(self.server_receive())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}
