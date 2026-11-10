use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceRole {
    Unassigned,
    XdgToplevel,
    XdgPopup,
    Subsurface,
    Cursor,
    LayerShell,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct SurfaceAttributes {
    pub buffer_id: Option<u32>,
    pub buffer_scale: i32,
    pub buffer_transform: i32,
    pub damage: Vec<Rect>,
    pub offset_x: i32,
    pub offset_y: i32,
    pub window_geometry: Rect,
}

impl Default for SurfaceAttributes {
    fn default() -> Self {
        Self {
            buffer_id: None,
            buffer_scale: 1,
            buffer_transform: 0,
            damage: Vec::new(),
            offset_x: 0,
            offset_y: 0,
            window_geometry: Rect::default(),
        }
    }
}

pub struct WaylandSurface {
    pub id: u32,
    pub role: SurfaceRole,
    pub pending: SurfaceAttributes,
    pub current: SurfaceAttributes,
    pub title: String,
    pub app_id: String,
    pub parent_id: Option<u32>,
    pub children: Vec<u32>,
    pub frame_callbacks: Vec<u32>,
    pub committed_frame_count: u64,
}

impl WaylandSurface {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            role: SurfaceRole::Unassigned,
            pending: SurfaceAttributes::default(),
            current: SurfaceAttributes::default(),
            title: String::new(),
            app_id: String::new(),
            parent_id: None,
            children: Vec::new(),
            frame_callbacks: Vec::new(),
            committed_frame_count: 0,
        }
    }

    pub fn commit(&mut self) -> u64 {
        self.current = self.pending.clone();
        self.pending.damage.clear();
        self.committed_frame_count += 1;
        self.committed_frame_count
    }

    pub fn attach_buffer(&mut self, buffer_id: u32, offset_x: i32, offset_y: i32) {
        self.pending.buffer_id = Some(buffer_id);
        self.pending.offset_x = offset_x;
        self.pending.offset_y = offset_y;
    }

    pub fn add_damage(&mut self, rect: Rect) {
        self.pending.damage.push(rect);
    }
}

pub struct SurfaceRegistry {
    pub surfaces: HashMap<u32, WaylandSurface>,
}

impl SurfaceRegistry {
    pub fn new() -> Self {
        Self {
            surfaces: HashMap::new(),
        }
    }

    pub fn create_surface(&mut self, id: u32) -> &mut WaylandSurface {
        self.surfaces.insert(id, WaylandSurface::new(id));
        self.surfaces.get_mut(&id).unwrap()
    }

    pub fn get_surface(&self, id: u32) -> Option<&WaylandSurface> {
        self.surfaces.get(&id)
    }

    pub fn get_surface_mut(&mut self, id: u32) -> Option<&mut WaylandSurface> {
        self.surfaces.get_mut(&id)
    }

    pub fn remove_surface(&mut self, id: u32) -> Option<WaylandSurface> {
        self.surfaces.remove(&id)
    }
}
