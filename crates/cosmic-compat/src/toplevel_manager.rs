use serde::{Deserialize, Serialize};
use smithay_wayland_wire::st::SurfaceRole;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicToplevelInfo {
    pub handle_id: u32,
    pub surface_id: u32,
    pub title: String,
    pub app_id: String,
    #[serde(skip)]
    pub role: Option<SurfaceRole>,
    pub is_active: bool,
    pub is_maximized: bool,
    pub is_minimized: bool,
    pub is_fullscreen: bool,
}

pub struct CosmicToplevelManager {
    pub toplevels: HashMap<u32, CosmicToplevelInfo>,
    pub active_handle: Option<u32>,
    pub next_handle_id: u32,
}

impl CosmicToplevelManager {
    pub fn new() -> Self {
        Self {
            toplevels: HashMap::new(),
            active_handle: None,
            next_handle_id: 1,
        }
    }

    pub fn register_toplevel(
        &mut self,
        surface_id: u32,
        title: impl Into<String>,
        app_id: impl Into<String>,
    ) -> u32 {
        let handle_id = self.next_handle_id;
        self.next_handle_id += 1;

        let info = CosmicToplevelInfo {
            handle_id,
            surface_id,
            title: title.into(),
            app_id: app_id.into(),
            role: Some(SurfaceRole::XdgToplevel),
            is_active: true,
            is_maximized: false,
            is_minimized: false,
            is_fullscreen: false,
        };

        self.toplevels.insert(handle_id, info);
        self.active_handle = Some(handle_id);
        handle_id
    }

    pub fn activate(&mut self, handle_id: u32) {
        if let Some(prev) = self.active_handle {
            if let Some(t) = self.toplevels.get_mut(&prev) {
                t.is_active = false;
            }
        }
        if let Some(t) = self.toplevels.get_mut(&handle_id) {
            t.is_active = true;
            self.active_handle = Some(handle_id);
        }
    }

    pub fn close(&mut self, handle_id: u32) -> Option<CosmicToplevelInfo> {
        if self.active_handle == Some(handle_id) {
            self.active_handle = None;
        }
        self.toplevels.remove(&handle_id)
    }
}
