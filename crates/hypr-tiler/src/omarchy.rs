use crate::geometry::WindowRect;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OmarchyAction {
    LaunchMenuRoot,
    ShowKeybindHud,
    SpawnTerminal(String),
    CloseActiveWindow,
    FocusWorkspace(u8),
    MoveWindowToWorkspace(u8),
    LiveSwitchAetherTheme(String),
    ToggleFloating,
    ToggleFullscreen,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmarchyKeybind {
    pub chord: String,
    pub description: String,
    pub action: OmarchyAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmarchyKeybindRegistry {
    pub bindings: BTreeMap<String, OmarchyKeybind>,
    pub active_theme: String,
}

impl Default for OmarchyKeybindRegistry {
    fn default() -> Self {
        Self::new("tokyo-night")
    }
}

impl OmarchyKeybindRegistry {
    pub fn new(default_theme: impl Into<String>) -> Self {
        let theme = default_theme.into();
        let mut reg = Self {
            bindings: BTreeMap::new(),
            active_theme: theme.clone(),
        };

        reg.register("SUPER+SPACE", "Omarchy root launcher menu", OmarchyAction::LaunchMenuRoot);
        reg.register("SUPER+K", "Omarchy keybindings HUD", OmarchyAction::ShowKeybindHud);
        reg.register(
            "SUPER+RETURN",
            "Spawn GPU-accelerated terminal",
            OmarchyAction::SpawnTerminal("ghostty".into()),
        );
        reg.register("SUPER+W", "Close active window", OmarchyAction::CloseActiveWindow);
        reg.register("SUPER+T", "Toggle window floating/tiling", OmarchyAction::ToggleFloating);
        reg.register("SUPER+F", "Toggle fullscreen", OmarchyAction::ToggleFullscreen);
        reg.register(
            "SUPER+SHIFT+T",
            "Live Aether theme hot-switch",
            OmarchyAction::LiveSwitchAetherTheme(theme),
        );

        for ws in 1..=9u8 {
            reg.register(
                &format!("SUPER+{}", ws),
                &format!("Switch to workspace {}", ws),
                OmarchyAction::FocusWorkspace(ws),
            );
            reg.register(
                &format!("SUPER+SHIFT+{}", ws),
                &format!("Move active window to workspace {}", ws),
                OmarchyAction::MoveWindowToWorkspace(ws),
            );
        }

        reg
    }

    pub fn register(
        &mut self,
        chord: impl Into<String>,
        description: impl Into<String>,
        action: OmarchyAction,
    ) {
        let key = Self::normalize_chord(&chord.into());
        self.bindings.insert(
            key.clone(),
            OmarchyKeybind {
                chord: key,
                description: description.into(),
                action,
            },
        );
    }

    pub fn normalize_chord(raw: &str) -> String {
        raw.split('+')
            .map(|p| p.trim().to_uppercase())
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>()
            .join("+")
    }

    pub fn resolve(&self, chord: &str) -> Option<&OmarchyAction> {
        let key = Self::normalize_chord(chord);
        self.bindings.get(&key).map(|b| &b.action)
    }

    pub fn switch_theme(&mut self, next_theme: impl Into<String>) {
        let next = next_theme.into();
        self.active_theme = next.clone();
        self.register(
            "SUPER+SHIFT+T",
            "Live Aether theme hot-switch",
            OmarchyAction::LiveSwitchAetherTheme(next),
        );
    }

    pub fn to_hyprland_conf(&self) -> String {
        let mut out = String::from("# Omarchy Keyboard-First Hyprland Bindings (Rust WSLg)\n");
        for b in self.bindings.values() {
            let parts: Vec<&str> = b.chord.split('+').collect();
            let (mods, key) = if parts.len() > 1 {
                (parts[..parts.len() - 1].join(" "), parts[parts.len() - 1])
            } else {
                (String::new(), parts[0])
            };
            let dispatcher = match &b.action {
                OmarchyAction::LaunchMenuRoot => "exec, omarchy-menu".to_string(),
                OmarchyAction::ShowKeybindHud => "exec, omarchy-menu-keybindings".to_string(),
                OmarchyAction::SpawnTerminal(term) => format!("exec, {}", term),
                OmarchyAction::CloseActiveWindow => "killactive,".to_string(),
                OmarchyAction::FocusWorkspace(ws) => format!("workspace, {}", ws),
                OmarchyAction::MoveWindowToWorkspace(ws) => format!("movetoworkspace, {}", ws),
                OmarchyAction::LiveSwitchAetherTheme(t) => {
                    format!("exec, omarchy-wsl themes export {}", t)
                }
                OmarchyAction::ToggleFloating => "togglefloating,".to_string(),
                OmarchyAction::ToggleFullscreen => "fullscreen, 0".to_string(),
            };
            out.push_str(&format!("bind = {}, {}, {} # {}\n", mods, key, dispatcher, b.description));
        }
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DxgDamageRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl DxgDamageRegion {
    pub fn union(&self, other: &Self) -> Self {
        let min_x = self.x.min(other.x);
        let min_y = self.y.min(other.y);
        let max_x = (self.x + self.width).max(other.x + other.width);
        let max_y = (self.y + self.height).max(other.y + other.height);
        Self {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxgSurfaceDamageState {
    pub surface_id: u32,
    pub dxg_allocation_handle: u64,
    pub nt_shared_handle: usize,
    pub dirty_rects: Vec<DxgDamageRegion>,
    pub committed_frames: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DxgDmaBufDamageTracker {
    pub surfaces: BTreeMap<u32, DxgSurfaceDamageState>,
    pub dxg_device_node: String,
}

impl DxgDmaBufDamageTracker {
    pub fn new() -> Self {
        Self {
            surfaces: BTreeMap::new(),
            dxg_device_node: "/dev/dxg".to_string(),
        }
    }

    pub fn register_surface(
        &mut self,
        surface_id: u32,
        dxg_allocation_handle: u64,
        nt_shared_handle: usize,
    ) {
        self.surfaces.insert(
            surface_id,
            DxgSurfaceDamageState {
                surface_id,
                dxg_allocation_handle,
                nt_shared_handle,
                dirty_rects: Vec::new(),
                committed_frames: 0,
            },
        );
    }

    pub fn record_damage(&mut self, surface_id: u32, rect: DxgDamageRegion) {
        if rect.width == 0 || rect.height == 0 {
            return;
        }
        if let Some(surf) = self.surfaces.get_mut(&surface_id) {
            surf.dirty_rects.push(rect);
        }
    }

    pub fn record_window_rect_damage(&mut self, surface_id: u32, win_rect: WindowRect) {
        let dmg = DxgDamageRegion {
            x: win_rect.x.max(0.0) as u32,
            y: win_rect.y.max(0.0) as u32,
            width: win_rect.width.max(1.0) as u32,
            height: win_rect.height.max(1.0) as u32,
        };
        self.record_damage(surface_id, dmg);
    }

    /// Coalesces pending `/dev/dxg` DMA-BUF damage rectangles into a minimal bounding scissor rect
    /// for zero-copy `ID3D12GraphicsCommandList::CopyTextureRegion` / DirectComposition presentation.
    pub fn drain_coalesced_damage(&mut self, surface_id: u32) -> Option<DxgDamageRegion> {
        let surf = self.surfaces.get_mut(&surface_id)?;
        if surf.dirty_rects.is_empty() {
            return None;
        }
        let mut iter = surf.dirty_rects.drain(..);
        let first = iter.next()?;
        let merged = iter.fold(first, |acc, r| acc.union(&r));
        surf.committed_frames += 1;
        Some(merged)
    }
}
