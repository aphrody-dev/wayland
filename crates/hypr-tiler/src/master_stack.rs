use crate::geometry::{GapsConfig, WindowRect};
use std::collections::HashMap;

pub struct MasterStackLayout {
    pub windows: Vec<u32>,
    pub master_count: usize,
    pub master_ratio: f64,
    pub gaps: GapsConfig,
}

impl MasterStackLayout {
    pub fn new(gaps: GapsConfig) -> Self {
        Self {
            windows: Vec::new(),
            master_count: 1,
            master_ratio: 0.55,
            gaps,
        }
    }

    pub fn add_window(&mut self, window_id: u32) {
        if !self.windows.contains(&window_id) {
            self.windows.push(window_id);
        }
    }

    pub fn remove_window(&mut self, window_id: u32) {
        self.windows.retain(|&id| id != window_id);
    }

    pub fn compute_layout(&self, screen_bounds: WindowRect) -> HashMap<u32, WindowRect> {
        let mut map = HashMap::new();
        if self.windows.is_empty() {
            return map;
        }

        let usable = WindowRect::new(
            screen_bounds.x + self.gaps.outer,
            screen_bounds.y + self.gaps.outer,
            screen_bounds.width - (self.gaps.outer * 2.0),
            screen_bounds.height - (self.gaps.outer * 2.0),
        );

        if self.windows.len() == 1 {
            let r = WindowRect::new(
                usable.x + self.gaps.inner,
                usable.y + self.gaps.inner,
                usable.width - (self.gaps.inner * 2.0),
                usable.height - (self.gaps.inner * 2.0),
            );
            map.insert(self.windows[0], r);
            return map;
        }

        let n = self.windows.len();
        let masters = self.master_count.min(n);
        let stack = n - masters;

        let master_width = if stack == 0 {
            usable.width
        } else {
            usable.width * self.master_ratio
        };
        let stack_width = usable.width - master_width;

        // Tile Master windows
        let master_h = usable.height / (masters as f64);
        for (i, &w_id) in self.windows.iter().take(masters).enumerate() {
            let rect = WindowRect::new(
                usable.x + self.gaps.inner,
                usable.y + (i as f64 * master_h) + self.gaps.inner,
                master_width - (self.gaps.inner * 2.0),
                master_h - (self.gaps.inner * 2.0),
            );
            map.insert(w_id, rect);
        }

        // Tile Stack windows
        if stack > 0 {
            let stack_h = usable.height / (stack as f64);
            for (i, &w_id) in self.windows.iter().skip(masters).enumerate() {
                let rect = WindowRect::new(
                    usable.x + master_width + self.gaps.inner,
                    usable.y + (i as f64 * stack_h) + self.gaps.inner,
                    stack_width - (self.gaps.inner * 2.0),
                    stack_h - (self.gaps.inner * 2.0),
                );
                map.insert(w_id, rect);
            }
        }

        map
    }
}
