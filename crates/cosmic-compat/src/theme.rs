use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmarchyPaletteBridge {
    pub name: String,
    pub is_dark: bool,
    pub accent_hex: String,
    pub background_hex: String,
    pub darker_background_hex: String,
    pub lighter_background_hex: String,
    pub foreground_hex: String,
    pub muted_hex: String,
}

impl OmarchyPaletteBridge {
    pub fn preset(name: &str) -> Self {
        match name.trim().to_lowercase().as_str() {
            "catppuccin" => Self {
                name: "catppuccin".into(),
                is_dark: true,
                accent_hex: "#89b4fa".into(),
                background_hex: "#1e1e2e".into(),
                darker_background_hex: "#101019".into(),
                lighter_background_hex: "#313244".into(),
                foreground_hex: "#cdd6f4".into(),
                muted_hex: "#585b70".into(),
            },
            "catppuccin-latte" => Self {
                name: "catppuccin-latte".into(),
                is_dark: false,
                accent_hex: "#1e66f5".into(),
                background_hex: "#eff1f5".into(),
                darker_background_hex: "#d7d8dc".into(),
                lighter_background_hex: "#dce0e8".into(),
                foreground_hex: "#4c4f69".into(),
                muted_hex: "#acb0be".into(),
            },
            "nord" => Self {
                name: "nord".into(),
                is_dark: true,
                accent_hex: "#81a1c1".into(),
                background_hex: "#2e3440".into(),
                darker_background_hex: "#191c23".into(),
                lighter_background_hex: "#3b4252".into(),
                foreground_hex: "#d8dee9".into(),
                muted_hex: "#4c566a".into(),
            },
            "gruvbox" => Self {
                name: "gruvbox".into(),
                is_dark: true,
                accent_hex: "#7daea3".into(),
                background_hex: "#282828".into(),
                darker_background_hex: "#161616".into(),
                lighter_background_hex: "#3c3836".into(),
                foreground_hex: "#d4be98".into(),
                muted_hex: "#665c54".into(),
            },
            "cachyos-emerald" => Self {
                name: "cachyos-emerald".into(),
                is_dark: true,
                accent_hex: "#00e8c6".into(),
                background_hex: "#0b1416".into(),
                darker_background_hex: "#050a0b".into(),
                lighter_background_hex: "#132326".into(),
                foreground_hex: "#e0f7f4".into(),
                muted_hex: "#486b68".into(),
            },
            "aphrody-m3-dark" => Self {
                name: "aphrody-m3-dark".into(),
                is_dark: true,
                accent_hex: "#a8c7fa".into(),
                background_hex: "#131314".into(),
                darker_background_hex: "#09090a".into(),
                lighter_background_hex: "#1f1f22".into(),
                foreground_hex: "#e3e3e3".into(),
                muted_hex: "#8e918f".into(),
            },
            other => Self {
                name: if other.is_empty() {
                    "tokyo-night".into()
                } else {
                    other.into()
                },
                is_dark: true,
                accent_hex: "#7aa2f7".into(),
                background_hex: "#1a1b26".into(),
                darker_background_hex: "#0e0e14".into(),
                lighter_background_hex: "#24283b".into(),
                foreground_hex: "#a9b1d6".into(),
                muted_hex: "#414868".into(),
            },
        }
    }
}

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

    pub fn from_hex_str(color: &str) -> Self {
        let stripped = color.trim().trim_start_matches('#');
        let parsed = u32::from_str_radix(stripped.get(0..6).unwrap_or("000000"), 16).unwrap_or(0);
        Self::from_hex(parsed)
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

    pub fn from_omarchy_palette(palette: &OmarchyPaletteBridge) -> Self {
        Self {
            name: format!("Omarchy Aether - {}", palette.name),
            is_dark: palette.is_dark,
            primary: CosmicColorRgba::from_hex_str(&palette.accent_hex),
            on_primary: CosmicColorRgba::from_hex_str(&palette.darker_background_hex),
            surface: CosmicColorRgba::from_hex_str(&palette.background_hex),
            on_surface: CosmicColorRgba::from_hex_str(&palette.foreground_hex),
            surface_container: CosmicColorRgba::from_hex_str(&palette.lighter_background_hex),
            outline: CosmicColorRgba::from_hex_str(&palette.muted_hex),
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
