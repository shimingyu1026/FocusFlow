use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusSession {
    pub id: String,
    pub task: String,
    pub duration: i32,        // 分钟
    #[serde(alias = "start_time")]
    pub start_time: String,   // ISO 8601
    #[serde(alias = "end_time")]
    pub end_time: String,     // ISO 8601
    pub completed: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub sound_enabled: bool,
    pub sound_volume: f64,
    pub default_duration: i32,
    pub theme_mode: String,
    pub theme_accent: String,
    pub celebration_style: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            sound_enabled: true,
            sound_volume: 0.7,
            default_duration: 25,
            theme_mode: "dark".to_string(),
            theme_accent: "ocean".to_string(),
            celebration_style: "confetti".to_string(),
        }
    }
}

impl AppSettings {
    pub fn sanitized(self) -> Self {
        let default = Self::default();

        Self {
            sound_enabled: self.sound_enabled,
            sound_volume: self.sound_volume.clamp(0.0, 1.0),
            default_duration: match self.default_duration {
                1..=480 => self.default_duration,
                _ => default.default_duration,
            },
            theme_mode: match self.theme_mode.as_str() {
                "dark" | "light" => self.theme_mode,
                _ => default.theme_mode,
            },
            theme_accent: match self.theme_accent.as_str() {
                "ocean" | "sunset" | "arcade" => self.theme_accent,
                _ => default.theme_accent,
            },
            celebration_style: match self.celebration_style.as_str() {
                "confetti" | "stars" | "fireworks" => self.celebration_style,
                _ => default.celebration_style,
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageLocations {
    pub database_path: String,
    pub settings_path: String,
    pub app_data_dir: String,
    pub app_config_dir: String,
}
