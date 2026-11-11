pub mod hidpi_protocols;
pub mod layer_shell;
pub mod session;
pub mod theme;
pub mod toplevel_manager;
pub mod workspace_manager;

pub use hidpi_protocols::{WslgHiDpiScalingDescriptor, WslgProtocolDescriptor};
pub use layer_shell::{LayerAnchor, LayerShellManager, LayerSurface, LayerType};
pub use session::CosmicSessionEnvironment;
pub use theme::{CosmicColorRgba, CosmicTheme, OmarchyPaletteBridge};
pub use toplevel_manager::{CosmicToplevelInfo, CosmicToplevelManager};
pub use workspace_manager::{CosmicWorkspaceInfo, CosmicWorkspaceManager};

#[cfg(test)]
mod tests {
    use super::*;
    use hypr_tiler::geometry::WindowRect;

    #[test]
    fn test_layer_shell_exclusive_zone_dock() {
        let mut shell = LayerShellManager::new();
        // Add cosmic-panel: top anchor, 48px height, 48px exclusive zone
        shell.add_surface(LayerSurface {
            id: 1,
            layer: LayerType::Top,
            anchor: LayerAnchor::TOP | LayerAnchor::LEFT | LayerAnchor::RIGHT,
            exclusive_zone: 48,
            width: 1920,
            height: 48,
            margin_top: 0,
            margin_bottom: 0,
            margin_left: 0,
            margin_right: 0,
        });

        let screen = WindowRect::new(0.0, 0.0, 1920.0, 1080.0);
        let usable = shell.calculate_usable_area(screen);

        assert_eq!(usable.x, 0.0);
        assert_eq!(usable.y, 48.0);
        assert_eq!(usable.width, 1920.0);
        assert_eq!(usable.height, 1032.0); // 1080 - 48
    }

    #[test]
    fn test_cosmic_toplevel_and_workspaces() {
        let mut toplevels = CosmicToplevelManager::new();
        let mut ws_mgr = CosmicWorkspaceManager::new();

        let handle1 = toplevels.register_toplevel(10, "COSMIC Terminal", "com.system76.CosmicTerm");
        let handle2 = toplevels.register_toplevel(11, "COSMIC Files", "com.system76.CosmicFiles");

        assert_eq!(toplevels.toplevels.len(), 2);
        assert_eq!(toplevels.active_handle, Some(handle2));

        // Switch active
        toplevels.activate(handle1);
        assert_eq!(toplevels.active_handle, Some(handle1));

        // Assign to workspace 2
        ws_mgr.assign_toplevel(2, handle1);
        assert_eq!(ws_mgr.workspaces.get(&2).unwrap().toplevel_handles, vec![handle1]);
    }

    #[test]
    fn test_cosmic_theme_material3_ron() {
        let theme = CosmicTheme::google_material_3_dark();
        assert_eq!(theme.name, "Google Material 3 Sovereign Dark");
        assert!(theme.is_dark);

        let ron = theme.to_ron_string();
        assert!(ron.contains("Google Material 3 Sovereign Dark"));
        assert!(ron.contains("primary: (red:"));
        assert!(ron.contains("corner_radius: 12.0"));
    }

    #[test]
    fn test_cosmic_session_environment() {
        let env = CosmicSessionEnvironment::new();
        let strings = env.to_env_strings();
        assert!(strings.contains(&"WGPU_BACKEND=vulkan".to_string()));
        assert!(strings.contains(&"COSMIC_DATA_CONTROL_ENABLED=1".to_string()));
        assert!(strings.contains(&"XDG_CURRENT_DESKTOP=COSMIC".to_string()));
    }

    #[test]
    fn test_omarchy_palette_bridge_and_wslg_fractional_hidpi_protocols() {
        let palette = OmarchyPaletteBridge::preset("tokyo-night");
        let cosmic_theme = CosmicTheme::from_omarchy_palette(&palette);
        assert_eq!(cosmic_theme.name, "Omarchy Aether - tokyo-night");
        assert!(cosmic_theme.is_dark);
        let ron = cosmic_theme.to_ron_string();
        assert!(ron.contains("Omarchy Aether - tokyo-night"));

        let latte = OmarchyPaletteBridge::preset("catppuccin-latte");
        let latte_theme = CosmicTheme::from_omarchy_palette(&latte);
        assert!(!latte_theme.is_dark);

        let hidpi = WslgHiDpiScalingDescriptor::from_windows_dpi(3840, 2160, 144); // 150% Windows 11 scale
        assert_eq!(hidpi.scale_factor_120ths, 180);
        assert_eq!(hidpi.logical_width, 2560);
        assert_eq!(hidpi.logical_height, 1440);
        assert!(hidpi.zero_blur_pixel_exact);
        assert_eq!(hidpi.protocols.len(), 4);
        assert!(hidpi
            .protocols
            .iter()
            .any(|p| p.interface == "ext_workspace_manager_v1"));
        assert!(hidpi
            .protocols
            .iter()
            .any(|p| p.interface == "zwp_linux_dmabuf_v1"));
        assert!(hidpi
            .protocols
            .iter()
            .any(|p| p.interface == "wp_fractional_scale_v1"));
        assert!(hidpi
            .protocols
            .iter()
            .any(|p| p.interface == "wp_viewporter"));
    }
}
