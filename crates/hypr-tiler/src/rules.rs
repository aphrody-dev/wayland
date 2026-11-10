use crate::geometry::WindowRect;
use crate::window::{HyprWindow, WindowState};

#[derive(Debug, Clone)]
pub enum WindowRuleAction {
    Float,
    Fullscreen,
    Workspace(String),
    Size(f64, f64),
    Opacity(f32),
    Pin,
}

#[derive(Debug, Clone)]
pub struct WindowRuleV2 {
    pub target_app_id: Option<String>,
    pub target_title: Option<String>,
    pub action: WindowRuleAction,
}

impl WindowRuleV2 {
    pub fn matches(&self, app_id: &str, title: &str) -> bool {
        if let Some(target_app) = &self.target_app_id {
            if !app_id.contains(target_app) {
                return false;
            }
        }
        if let Some(target_t) = &self.target_title {
            if !title.contains(target_t) {
                return false;
            }
        }
        true
    }

    pub fn apply(&self, window: &mut HyprWindow) {
        match &self.action {
            WindowRuleAction::Float => window.state = WindowState::Floating,
            WindowRuleAction::Fullscreen => window.state = WindowState::Fullscreen,
            WindowRuleAction::Workspace(ws) => window.workspace = ws.clone(),
            WindowRuleAction::Size(w, h) => {
                window.target_geometry = WindowRect::new(
                    window.target_geometry.x,
                    window.target_geometry.y,
                    *w,
                    *h,
                );
            }
            WindowRuleAction::Opacity(op) => window.opacity = *op,
            WindowRuleAction::Pin => window.state = WindowState::Pinned,
        }
    }
}

pub struct WindowRulesEngine {
    pub rules: Vec<WindowRuleV2>,
}

impl WindowRulesEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: WindowRuleV2) {
        self.rules.push(rule);
    }

    pub fn process_window(&self, window: &mut HyprWindow) {
        for rule in &self.rules {
            if rule.matches(&window.app_id, &window.title) {
                rule.apply(window);
            }
        }
    }
}
