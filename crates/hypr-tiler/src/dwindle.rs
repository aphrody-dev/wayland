use crate::geometry::{GapsConfig, WindowRect};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal, // Top / Bottom
    Vertical,   // Left / Right
}

#[derive(Debug, Clone)]
pub enum BspNode {
    Leaf {
        window_id: u32,
    },
    Split {
        direction: SplitDirection,
        ratio: f64, // e.g. 0.5
        left: Box<BspNode>,
        right: Box<BspNode>,
    },
}

impl BspNode {
    pub fn new_leaf(window_id: u32) -> Self {
        BspNode::Leaf { window_id }
    }

    pub fn insert(&mut self, new_window_id: u32, target_window_id: Option<u32>) -> bool {
        match self {
            BspNode::Leaf { window_id } => {
                if target_window_id.is_none() || target_window_id == Some(*window_id) {
                    let old_id = *window_id;
                    *self = BspNode::Split {
                        direction: SplitDirection::Vertical,
                        ratio: 0.5,
                        left: Box::new(BspNode::Leaf { window_id: old_id }),
                        right: Box::new(BspNode::Leaf {
                            window_id: new_window_id,
                        }),
                    };
                    return true;
                }
                false
            }
            BspNode::Split {
                direction,
                left,
                right,
                ..
            } => {
                // Determine next split direction alternated (dwindle spiral behavior)
                let _ = direction;
                if left.insert(new_window_id, target_window_id) {
                    return true;
                }
                right.insert(new_window_id, target_window_id)
            }
        }
    }

    pub fn remove(&mut self, target_window_id: u32) -> (bool, Option<Box<BspNode>>) {
        match self {
            BspNode::Leaf { window_id } => {
                if *window_id == target_window_id {
                    (true, None)
                } else {
                    (false, None)
                }
            }
            BspNode::Split { left, right, .. } => {
                let (left_removed, left_repl) = left.remove(target_window_id);
                if left_removed {
                    if let Some(repl) = left_repl {
                        *left = repl;
                    } else {
                        // Sibling takes over this node
                        return (true, Some(right.clone()));
                    }
                    return (true, None);
                }

                let (right_removed, right_repl) = right.remove(target_window_id);
                if right_removed {
                    if let Some(repl) = right_repl {
                        *right = repl;
                    } else {
                        // Left takes over this node
                        return (true, Some(left.clone()));
                    }
                    return (true, None);
                }

                (false, None)
            }
        }
    }

    pub fn calculate_layout(
        &self,
        bounds: WindowRect,
        gaps: &GapsConfig,
        out: &mut HashMap<u32, WindowRect>,
    ) {
        match self {
            BspNode::Leaf { window_id } => {
                let rect = WindowRect::new(
                    bounds.x + gaps.inner,
                    bounds.y + gaps.inner,
                    bounds.width - (gaps.inner * 2.0),
                    bounds.height - (gaps.inner * 2.0),
                );
                out.insert(*window_id, rect);
            }
            BspNode::Split {
                direction,
                ratio,
                left,
                right,
            } => {
                let r = ratio.clamp(0.1, 0.9);
                match direction {
                    SplitDirection::Vertical => {
                        let left_width = bounds.width * r;
                        let right_width = bounds.width - left_width;

                        let left_bounds = WindowRect::new(bounds.x, bounds.y, left_width, bounds.height);
                        let right_bounds = WindowRect::new(
                            bounds.x + left_width,
                            bounds.y,
                            right_width,
                            bounds.height,
                        );

                        left.calculate_layout(left_bounds, gaps, out);
                        right.calculate_layout(right_bounds, gaps, out);
                    }
                    SplitDirection::Horizontal => {
                        let top_height = bounds.height * r;
                        let bottom_height = bounds.height - top_height;

                        let top_bounds = WindowRect::new(bounds.x, bounds.y, bounds.width, top_height);
                        let bottom_bounds = WindowRect::new(
                            bounds.x,
                            bounds.y + top_height,
                            bounds.width,
                            bottom_height,
                        );

                        left.calculate_layout(top_bounds, gaps, out);
                        right.calculate_layout(bottom_bounds, gaps, out);
                    }
                }
            }
        }
    }
}

pub struct DwindleTree {
    pub root: Option<BspNode>,
    pub gaps: GapsConfig,
}

impl DwindleTree {
    pub fn new(gaps: GapsConfig) -> Self {
        Self { root: None, gaps }
    }

    pub fn insert_window(&mut self, window_id: u32, focus: Option<u32>) {
        if let Some(root) = &mut self.root {
            root.insert(window_id, focus);
        } else {
            self.root = Some(BspNode::new_leaf(window_id));
        }
    }

    pub fn remove_window(&mut self, window_id: u32) {
        if let Some(mut root) = self.root.take() {
            let (removed, repl) = root.remove(window_id);
            if removed {
                if let Some(r) = repl {
                    self.root = Some(*r);
                } else {
                    // Tree was just that one leaf
                    self.root = None;
                }
            } else {
                self.root = Some(root);
            }
        }
    }

    pub fn compute_layout(&self, screen_bounds: WindowRect) -> HashMap<u32, WindowRect> {
        let mut map = HashMap::new();
        if let Some(root) = &self.root {
            // Apply outer gaps to the screen bounds
            let usable_bounds = WindowRect::new(
                screen_bounds.x + self.gaps.outer,
                screen_bounds.y + self.gaps.outer,
                screen_bounds.width - (self.gaps.outer * 2.0),
                screen_bounds.height - (self.gaps.outer * 2.0),
            );
            root.calculate_layout(usable_bounds, &self.gaps, &mut map);
        }
        map
    }
}
