use std::collections::HashMap;

pub struct CosmicSessionEnvironment {
    pub env_vars: HashMap<String, String>,
}

impl CosmicSessionEnvironment {
    pub fn new() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("WGPU_BACKEND".into(), "vulkan".into());
        env_vars.insert("COSMIC_DATA_CONTROL_ENABLED".into(), "1".into());
        env_vars.insert("WAYLAND_DISPLAY".into(), "wayland-0".into());
        env_vars.insert("XDG_CURRENT_DESKTOP".into(), "COSMIC".into());
        env_vars.insert("XDG_SESSION_DESKTOP".into(), "COSMIC".into());
        env_vars.insert("XDG_SESSION_TYPE".into(), "wayland".into());
        env_vars.insert("GDK_BACKEND".into(), "wayland".into());
        env_vars.insert("QT_QPA_PLATFORM".into(), "wayland".into());
        env_vars.insert("SDL_VIDEODRIVER".into(), "wayland".into());
        env_vars.insert("CLUTTER_BACKEND".into(), "wayland".into());

        Self { env_vars }
    }

    pub fn to_env_strings(&self) -> Vec<String> {
        let mut list: Vec<String> = self
            .env_vars
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        list.sort();
        list
    }
}
