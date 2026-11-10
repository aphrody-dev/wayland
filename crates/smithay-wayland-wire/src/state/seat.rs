pub const SEAT_CAPABILITY_POINTER: u32 = 1;
pub const SEAT_CAPABILITY_KEYBOARD: u32 = 2;
pub const SEAT_CAPABILITY_TOUCH: u32 = 4;

#[derive(Debug, Clone)]
pub struct PointerState {
    pub surface_id: Option<u32>,
    pub x: f64,
    pub y: f64,
    pub button_mask: u32,
}

#[derive(Debug, Clone)]
pub struct KeyboardModifiers {
    pub depressed: u32,
    pub latched: u32,
    pub locked: u32,
    pub group: u32,
}

#[derive(Debug, Clone)]
pub struct KeyboardState {
    pub focused_surface_id: Option<u32>,
    pub modifiers: KeyboardModifiers,
    pub repeat_rate: i32,
    pub repeat_delay: i32,
}

pub struct WaylandSeat {
    pub name: String,
    pub capabilities: u32,
    pub pointer: PointerState,
    pub keyboard: KeyboardState,
}

impl WaylandSeat {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            capabilities: SEAT_CAPABILITY_POINTER | SEAT_CAPABILITY_KEYBOARD | SEAT_CAPABILITY_TOUCH,
            pointer: PointerState {
                surface_id: None,
                x: 0.0,
                y: 0.0,
                button_mask: 0,
            },
            keyboard: KeyboardState {
                focused_surface_id: None,
                modifiers: KeyboardModifiers {
                    depressed: 0,
                    latched: 0,
                    locked: 0,
                    group: 0,
                },
                repeat_rate: 25,
                repeat_delay: 200,
            },
        }
    }
}
