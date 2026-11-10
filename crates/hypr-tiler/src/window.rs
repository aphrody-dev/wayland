use crate::geometry::WindowRect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowState {
    Tiled,
    Floating,
    Fullscreen,
    Pinned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyprWindow {
    pub id: u32,
    pub app_id: String,
    pub title: String,
    pub state: WindowState,
    pub workspace: String,
    pub geometry: WindowRect,
    pub target_geometry: WindowRect,
    pub opacity: f32,
}

impl HyprWindow {
    pub fn new(id: u32, app_id: impl Into<String>, title: impl Into<String>, workspace: impl Into<String>) -> Self {
        Self {
            id,
            app_id: app_id.into(),
            title: title.into(),
            state: WindowState::Tiled,
            workspace: workspace.into(),
            geometry: WindowRect::new(0.0, 0.0, 800.0, 600.0),
            target_geometry: WindowRect::new(0.0, 0.0, 800.0, 600.0),
            opacity: 1.0,
        }
    }
}
