use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Skyset {
    #[serde(rename = "_version")]
    pub version: u32,
    pub origin: String,
    pub updated_at: String,
    pub message: String,
    pub submessage: String,
    pub source_will_update: bool,
    pub theme: Theme,
    pub palette: Palette,
    pub gradients: Gradients,
}

impl Default for Skyset {
    fn default() -> Self {
        Self {
            version: SCHEMA_VERSION,
            origin: String::new(),
            updated_at: String::new(),
            message: String::new(),
            submessage: String::new(),
            source_will_update: true,
            theme: Theme::default(),
            palette: Palette::default(),
            gradients: Gradients::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Theme {
    pub mode: ThemeMode,
    pub accent: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Dark,
            accent: "#7C4DFF".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Dark,
    Light,
    System,
    #[serde(other)]
    Unknown,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Dark
    }
}

pub fn theme_mode_label(mode: ThemeMode) -> &'static str {
    match mode {
        ThemeMode::Dark => "dark",
        ThemeMode::Light => "light",
        ThemeMode::System => "system",
        ThemeMode::Unknown => "unknown",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Palette {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            primary: "#0E0E10".to_string(),
            secondary: "#1F1F23".to_string(),
            tertiary: "#2E2E35".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Gradients {
    pub background: Vec<String>,
    pub hero: Vec<String>,
}

impl Default for Gradients {
    fn default() -> Self {
        Self {
            background: vec![
                "#0B0B12".to_string(),
                "#141424".to_string(),
                "#1D1D32".to_string(),
            ],
            hero: vec!["#7C4DFF".to_string(), "#5E35B1".to_string()],
        }
    }
}
