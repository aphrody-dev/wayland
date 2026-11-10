use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicColorRgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl CosmicColorRgba {
    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicTheme {
    pub name: String,
    pub is_dark: bool,
    pub primary: CosmicColorRgba,
    pub on_primary: CosmicColorRgba,
    pub surface: CosmicColorRgba,
    pub on_surface: CosmicColorRgba,
    pub surface_container: CosmicColorRgba,
    pub outline: CosmicColorRgba,
    pub corner_radius: f32,
    pub spacing: f32,
}

impl CosmicTheme {
    pub fn google_material_3_dark() -> Self {
        Self {
            name: "Google Material 3 Sovereign Dark".into(),
            is_dark: true,
            primary: CosmicColorRgba::from_hex(0xA8C7FA),           // Material 3 Primary Blue
            on_primary: CosmicColorRgba::from_hex(0x00315B),        // M3 On Primary
            surface: CosmicColorRgba::from_hex(0x1F1F1F),           // M3 Dark Surface
            on_surface: CosmicColorRgba::from_hex(0xE3E3E3),        // M3 On Surface
            surface_container: CosmicColorRgba::from_hex(0x2B2B2B), // M3 Surface Container
            outline: CosmicColorRgba::from_hex(0x8E918F),           // M3 Outline
            corner_radius: 12.0,
            spacing: 8.0,
        }
    }

    pub fn to_ron_string(&self) -> String {
        format!(
            "(\n\
            \x20   name: \"{}\",\n\
            \x20   is_dark: {},\n\
            \x20   primary: (red: {:.4}, green: {:.4}, blue: {:.4}, alpha: {:.4}),\n\
            \x20   on_primary: (red: {:.4}, green: {:.4}, blue: {:.4}, alpha: {:.4}),\n\
            \x20   surface: (red: {:.4}, green: {:.4}, blue: {:.4}, alpha: {:.4}),\n\
            \x20   on_surface: (red: {:.4}, green: {:.4}, blue: {:.4}, alpha: {:.4}),\n\
            \x20   surface_container: (red: {:.4}, green: {:.4}, blue: {:.4}, alpha: {:.4}),\n\
            \x20   outline: (red: {:.4}, green: {:.4}, blue: {:.4}, alpha: {:.4}),\n\
            \x20   corner_radius: {:.1},\n\
            \x20   spacing: {:.1},\n\
            )\n",
            self.name,
            self.is_dark,
            self.primary.red, self.primary.green, self.primary.blue, self.primary.alpha,
            self.on_primary.red, self.on_primary.green, self.on_primary.blue, self.on_primary.alpha,
            self.surface.red, self.surface.green, self.surface.blue, self.surface.alpha,
            self.on_surface.red, self.on_surface.green, self.on_surface.blue, self.on_surface.alpha,
            self.surface_container.red, self.surface_container.green, self.surface_container.blue, self.surface_container.alpha,
            self.outline.red, self.outline.green, self.outline.blue, self.outline.alpha,
            self.corner_radius,
            self.spacing
        )
    }
}
