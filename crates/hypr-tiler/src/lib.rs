pub mod animation;
pub mod dwindle;
pub mod geometry;
pub mod master_stack;
pub mod omarchy;
pub mod rules;
pub mod window;
pub mod workspace;

pub use animation::{Spring1D, SpringParams, WindowGeometryAnimator};
pub use dwindle::{BspNode, DwindleTree, SplitDirection};
pub use geometry::{BorderConfig, GapsConfig, WindowRect};
pub use master_stack::MasterStackLayout;
pub use omarchy::{
    DxgDamageRegion, DxgDmaBufDamageTracker, DxgSurfaceDamageState, OmarchyAction, OmarchyKeybind,
    OmarchyKeybindRegistry,
};
pub use rules::{WindowRuleAction, WindowRuleV2, WindowRulesEngine};
pub use window::{HyprWindow, WindowState};
pub use workspace::Workspace;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dwindle_tiling_tree() {
        let gaps = GapsConfig {
            inner: 5.0,
            outer: 10.0,
        };
        let mut dwindle = DwindleTree::new(gaps);

        // Insert first window (fullscreen-minus-gaps)
        dwindle.insert_window(1, None);
        let screen = WindowRect::new(0.0, 0.0, 1920.0, 1080.0);
        let layout = dwindle.compute_layout(screen);
        assert_eq!(layout.len(), 1);

        let w1 = layout.get(&1).unwrap();
        assert_eq!(w1.x, 15.0); // outer(10) + inner(5)
        assert_eq!(w1.y, 15.0);
        assert_eq!(w1.width, 1890.0); // 1920 - 20 - 10
        assert_eq!(w1.height, 1050.0); // 1080 - 20 - 10

        // Insert second window (splits horizontally/vertically)
        dwindle.insert_window(2, Some(1));
        let layout2 = dwindle.compute_layout(screen);
        assert_eq!(layout2.len(), 2);
        let w1_split = layout2.get(&1).unwrap();
        let w2_split = layout2.get(&2).unwrap();
        assert!(w1_split.width < w1.width);
        assert!(w2_split.x > w1_split.x);

        // Remove window 1 (window 2 expands back to full pane)
        dwindle.remove_window(1);
        let layout3 = dwindle.compute_layout(screen);
        assert_eq!(layout3.len(), 1);
        let w2_solo = layout3.get(&2).unwrap();
        assert_eq!(w2_solo.width, 1890.0);
    }

    #[test]
    fn test_master_stack_layout() {
        let gaps = GapsConfig {
            inner: 5.0,
            outer: 10.0,
        };
        let mut ms = MasterStackLayout::new(gaps);
        ms.add_window(10);
        ms.add_window(20);
        ms.add_window(30);

        let screen = WindowRect::new(0.0, 0.0, 1920.0, 1080.0);
        let layout = ms.compute_layout(screen);
        assert_eq!(layout.len(), 3);

        let master = layout.get(&10).unwrap();
        let stack1 = layout.get(&20).unwrap();
        let stack2 = layout.get(&30).unwrap();

        assert!(master.width > stack1.width);
        assert_eq!(stack1.width, stack2.width);
        assert_eq!(stack1.x, stack2.x);
        assert!(stack2.y > stack1.y);
    }

    #[test]
    fn test_windowrulev2_matching() {
        let mut engine = WindowRulesEngine::new();
        engine.add_rule(WindowRuleV2 {
            target_app_id: Some("org.mozilla.firefox".into()),
            target_title: None,
            action: WindowRuleAction::Workspace("2".into()),
        });
        engine.add_rule(WindowRuleV2 {
            target_app_id: None,
            target_title: Some("Picture-in-Picture".into()),
            action: WindowRuleAction::Float,
        });

        let mut win1 = HyprWindow::new(1, "org.mozilla.firefox", "Mozilla Firefox", "1");
        engine.process_window(&mut win1);
        assert_eq!(win1.workspace, "2");
        assert_eq!(win1.state, WindowState::Tiled);

        let mut win2 = HyprWindow::new(2, "org.mozilla.firefox", "Picture-in-Picture", "1");
        engine.process_window(&mut win2);
        assert_eq!(win2.workspace, "2");
        assert_eq!(win2.state, WindowState::Floating);
    }

    #[test]
    fn test_spring_animation_convergence() {
        let initial = WindowRect::new(0.0, 0.0, 100.0, 100.0);
        let target = WindowRect::new(500.0, 200.0, 800.0, 600.0);
        let mut anim = WindowGeometryAnimator::new(initial, target, SpringParams::default());

        // Step 120Hz (dt = 1/120s = ~0.00833s) for 60 frames (~0.5s)
        let dt = 1.0 / 120.0;
        for _ in 0..120 {
            anim.step(dt);
        }

        assert!(anim.is_settled());
        let final_rect = anim.step(dt);
        assert!((final_rect.x - target.x).abs() < 1.0);
        assert!((final_rect.y - target.y).abs() < 1.0);
        assert!((final_rect.width - target.width).abs() < 1.0);
        assert!((final_rect.height - target.height).abs() < 1.0);
    }

    #[test]
    fn test_omarchy_keybindings_and_dxg_dmabuf_damage_coalescing() {
        let mut reg = OmarchyKeybindRegistry::new("tokyo-night");
        assert_eq!(reg.resolve("Super + Space"), Some(&OmarchyAction::LaunchMenuRoot));
        assert_eq!(reg.resolve("Super + K"), Some(&OmarchyAction::ShowKeybindHud));
        assert_eq!(
            reg.resolve("Super + Return"),
            Some(&OmarchyAction::SpawnTerminal("ghostty".into()))
        );
        assert_eq!(reg.resolve("Super + W"), Some(&OmarchyAction::CloseActiveWindow));
        for ws in 1..=9u8 {
            assert_eq!(
                reg.resolve(&format!("Super + {}", ws)),
                Some(&OmarchyAction::FocusWorkspace(ws))
            );
        }
        assert_eq!(
            reg.resolve("Super + Shift + T"),
            Some(&OmarchyAction::LiveSwitchAetherTheme("tokyo-night".into()))
        );

        reg.switch_theme("catppuccin");
        assert_eq!(
            reg.resolve("Super + Shift + T"),
            Some(&OmarchyAction::LiveSwitchAetherTheme("catppuccin".into()))
        );

        let conf = reg.to_hyprland_conf();
        assert!(conf.contains("omarchy-menu"));
        assert!(conf.contains("omarchy-wsl themes export catppuccin"));

        let mut tracker = DxgDmaBufDamageTracker::new();
        assert_eq!(tracker.dxg_device_node, "/dev/dxg");
        tracker.register_surface(42, 0xDEAD_BEEF, 0x1234);
        tracker.record_damage(
            42,
            DxgDamageRegion {
                x: 10,
                y: 20,
                width: 100,
                height: 50,
            },
        );
        tracker.record_damage(
            42,
            DxgDamageRegion {
                x: 80,
                y: 40,
                width: 120,
                height: 60,
            },
        );
        let coalesced = tracker.drain_coalesced_damage(42).unwrap();
        assert_eq!(coalesced.x, 10);
        assert_eq!(coalesced.y, 20);
        assert_eq!(coalesced.width, 190);
        assert_eq!(coalesced.height, 80);
        assert_eq!(tracker.surfaces.get(&42).unwrap().committed_frames, 1);
        assert!(tracker.drain_coalesced_damage(42).is_none());
    }
}
