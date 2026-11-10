use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WindowRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl WindowRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width: width.max(1.0),
            height: height.max(1.0),
        }
    }

    pub fn apply_gaps(&self, inner: f64, outer: f64) -> Self {
        let x = self.x + outer + inner;
        let y = self.y + outer + inner;
        let width = (self.width - (outer * 2.0) - (inner * 2.0)).max(10.0);
        let height = (self.height - (outer * 2.0) - (inner * 2.0)).max(10.0);
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GapsConfig {
    pub inner: f64,
    pub outer: f64,
}

impl Default for GapsConfig {
    fn default() -> Self {
        Self {
            inner: 5.0,
            outer: 10.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BorderConfig {
    pub width: f64,
    pub active_color: u32,
    pub inactive_color: u32,
    pub gradient_angle_deg: f32,
}

impl Default for BorderConfig {
    fn default() -> Self {
        Self {
            width: 2.0,
            active_color: 0xFF33CCFF,   // Hypr cyan gradient
            inactive_color: 0xFF595959, // Charcoal
            gradient_angle_deg: 45.0,
        }
    }
}
