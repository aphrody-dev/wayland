use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub name: String,
    pub is_special: bool,
    pub windows: HashSet<u32>,
}

impl Workspace {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let is_special = name.starts_with("special:");
        Self {
            name,
            is_special,
            windows: HashSet::new(),
        }
    }

    pub fn add_window(&mut self, window_id: u32) {
        self.windows.insert(window_id);
    }

    pub fn remove_window(&mut self, window_id: u32) {
        self.windows.remove(&window_id);
    }
}
