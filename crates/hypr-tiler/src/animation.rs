use crate::geometry::WindowRect;

#[derive(Debug, Clone, Copy)]
pub struct SpringParams {
    pub mass: f64,
    pub stiffness: f64,
    pub damping: f64,
}

impl Default for SpringParams {
    fn default() -> Self {
        // High-responsiveness 120Hz snappy spring profile (Hyprland default feels)
        Self {
            mass: 1.0,
            stiffness: 400.0,
            damping: 35.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Spring1D {
    pub position: f64,
    pub velocity: f64,
    pub target: f64,
    pub params: SpringParams,
}

impl Spring1D {
    pub fn new(initial: f64, target: f64, params: SpringParams) -> Self {
        Self {
            position: initial,
            velocity: 0.0,
            target,
            params,
        }
    }

    pub fn step(&mut self, dt: f64) {
        // Semi-implicit Euler integration for real-time physics stability
        let displacement = self.position - self.target;
        let spring_force = -self.params.stiffness * displacement;
        let damping_force = -self.params.damping * self.velocity;
        let acceleration = (spring_force + damping_force) / self.params.mass;

        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
    }

    pub fn is_settled(&self, tolerance: f64) -> bool {
        (self.position - self.target).abs() < tolerance && self.velocity.abs() < (tolerance * 10.0)
    }
}

pub struct WindowGeometryAnimator {
    pub x: Spring1D,
    pub y: Spring1D,
    pub width: Spring1D,
    pub height: Spring1D,
}

impl WindowGeometryAnimator {
    pub fn new(current: WindowRect, target: WindowRect, params: SpringParams) -> Self {
        Self {
            x: Spring1D::new(current.x, target.x, params),
            y: Spring1D::new(current.y, target.y, params),
            width: Spring1D::new(current.width, target.width, params),
            height: Spring1D::new(current.height, target.height, params),
        }
    }

    pub fn set_target(&mut self, target: WindowRect) {
        self.x.target = target.x;
        self.y.target = target.y;
        self.width.target = target.width;
        self.height.target = target.height;
    }

    pub fn step(&mut self, dt: f64) -> WindowRect {
        self.x.step(dt);
        self.y.step(dt);
        self.width.step(dt);
        self.height.step(dt);

        WindowRect::new(
            self.x.position,
            self.y.position,
            self.width.position,
            self.height.position,
        )
    }

    pub fn is_settled(&self) -> bool {
        self.x.is_settled(1.0)
            && self.y.is_settled(1.0)
            && self.width.is_settled(1.0)
            && self.height.is_settled(1.0)
    }
}
