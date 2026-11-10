use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CosmicWorkspaceInfo {
    pub id: u32,
    pub name: String,
    pub is_active: bool,
    pub toplevel_handles: Vec<u32>,
}

pub struct CosmicWorkspaceManager {
    pub workspaces: HashMap<u32, CosmicWorkspaceInfo>,
    pub active_workspace_id: u32,
}

impl CosmicWorkspaceManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            workspaces: HashMap::new(),
            active_workspace_id: 1,
        };
        // Initial 4 default COSMIC workspaces
        for i in 1..=4 {
            mgr.workspaces.insert(
                i,
                CosmicWorkspaceInfo {
                    id: i,
                    name: format!("Workspace {}", i),
                    is_active: i == 1,
                    toplevel_handles: Vec::new(),
                },
            );
        }
        mgr
    }

    pub fn set_active(&mut self, id: u32) {
        if self.workspaces.contains_key(&id) {
            if let Some(prev) = self.workspaces.get_mut(&self.active_workspace_id) {
                prev.is_active = false;
            }
            if let Some(curr) = self.workspaces.get_mut(&id) {
                curr.is_active = true;
                self.active_workspace_id = id;
            }
        }
    }

    pub fn assign_toplevel(&mut self, workspace_id: u32, handle_id: u32) {
        // Remove from existing
        for ws in self.workspaces.values_mut() {
            ws.toplevel_handles.retain(|&h| h != handle_id);
        }
        if let Some(ws) = self.workspaces.get_mut(&workspace_id) {
            ws.toplevel_handles.push(handle_id);
        }
    }
}
