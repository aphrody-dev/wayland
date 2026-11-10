use hypr_tiler::geometry::WindowRect;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerType {
    Background, // cosmic-bg
    Bottom,
    Top,     // cosmic-panel, cosmic-applets
    Overlay, // cosmic-launcher, cosmic-osd
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LayerAnchor: u32 {
        const TOP = 1;
        const BOTTOM = 2;
        const LEFT = 4;
        const RIGHT = 8;
    }
}

#[derive(Debug, Clone)]
pub struct LayerSurface {
    pub id: u32,
    pub layer: LayerType,
    pub anchor: LayerAnchor,
    pub exclusive_zone: i32, // e.g. 48px panel
    pub width: u32,
    pub height: u32,
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
    pub margin_right: i32,
}

pub struct LayerShellManager {
    pub surfaces: HashMap<u32, LayerSurface>,
}

impl LayerShellManager {
    pub fn new() -> Self {
        Self {
            surfaces: HashMap::new(),
        }
    }

    pub fn add_surface(&mut self, surface: LayerSurface) {
        self.surfaces.insert(surface.id, surface);
    }

    pub fn remove_surface(&mut self, id: u32) {
        self.surfaces.remove(&id);
    }

    /// Calculates usable workspace area for tiling windows after reserving exclusive zones (e.g. cosmic-panel)
    pub fn calculate_usable_area(&self, screen_bounds: WindowRect) -> WindowRect {
        let mut x = screen_bounds.x;
        let mut y = screen_bounds.y;
        let mut width = screen_bounds.width;
        let mut height = screen_bounds.height;

        for surf in self.surfaces.values() {
            if surf.exclusive_zone > 0 {
                let ez = surf.exclusive_zone as f64;
                if surf.anchor.contains(LayerAnchor::TOP) {
                    y += ez;
                    height -= ez;
                } else if surf.anchor.contains(LayerAnchor::BOTTOM) {
                    height -= ez;
                } else if surf.anchor.contains(LayerAnchor::LEFT) {
                    x += ez;
                    width -= ez;
                } else if surf.anchor.contains(LayerAnchor::RIGHT) {
                    width -= ez;
                }
            }
        }

        WindowRect::new(x, y, width.max(100.0), height.max(100.0))
    }
}
