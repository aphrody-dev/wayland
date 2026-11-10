#[derive(Debug, Clone)]
pub struct OutputMode {
    pub width: i32,
    pub height: i32,
    pub refresh_mhz: i32, // e.g. 120000 = 120Hz, 144000 = 144Hz
    pub is_current: bool,
    pub is_preferred: bool,
}

#[derive(Debug, Clone)]
pub struct WaylandOutput {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub physical_width_mm: i32,
    pub physical_height_mm: i32,
    pub modes: Vec<OutputMode>,
    pub scale_factor: i32,
    pub fractional_scale: u32, // 120 = 1.0x, 150 = 1.25x, 180 = 1.5x (in 120ths per wp_fractional_scale_v1)
}

impl WaylandOutput {
    pub fn new_rtx4070_120hz(id: u32) -> Self {
        Self {
            id,
            name: "DP-1".into(),
            description: "NVIDIA GeForce RTX 4070 High-Performance Display".into(),
            make: "Antigravity Sovereign".into(),
            model: "RTX-4070-120HZ".into(),
            physical_width_mm: 600,
            physical_height_mm: 340,
            modes: vec![
                OutputMode {
                    width: 2560,
                    height: 1440,
                    refresh_mhz: 120000,
                    is_current: true,
                    is_preferred: true,
                },
                OutputMode {
                    width: 3840,
                    height: 2160,
                    refresh_mhz: 120000,
                    is_current: false,
                    is_preferred: false,
                },
                OutputMode {
                    width: 1920,
                    height: 1080,
                    refresh_mhz: 144000,
                    is_current: false,
                    is_preferred: false,
                },
            ],
            scale_factor: 1,
            fractional_scale: 120, // 1.0x
        }
    }
}
