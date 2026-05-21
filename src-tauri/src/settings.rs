use crate::models::AppSettings;
use std::fs;
use std::path::{Path, PathBuf};

const SETTINGS_NAME: &str = "settings.json";

pub fn get_settings_path(app_config_dir: PathBuf) -> PathBuf {
    app_config_dir.join(SETTINGS_NAME)
}

pub fn load_settings(settings_path: &Path) -> Result<AppSettings, String> {
    if !settings_path.exists() {
        return Ok(AppSettings::default());
    }

    let raw = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: AppSettings = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    Ok(settings.sanitized())
}

pub fn save_settings(settings_path: &Path, settings: AppSettings) -> Result<AppSettings, String> {
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let settings = settings.sanitized();
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(settings_path, json).map_err(|e| e.to_string())?;
    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_settings_path(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        std::env::temp_dir()
            .join(format!("focusflow-settings-{test_name}-{unique}"))
            .join("settings.json")
    }

    #[test]
    fn missing_file_returns_defaults() {
        let path = temp_settings_path("missing");
        let settings = load_settings(&path).expect("missing settings should load defaults");

        assert!(settings.sound_enabled);
        assert_eq!(settings.default_duration, 25);
    }

    #[test]
    fn save_and_load_settings_roundtrip() {
        let path = temp_settings_path("roundtrip");
        let settings = AppSettings {
            sound_enabled: false,
            sound_volume: 0.4,
            default_duration: 45,
            theme_mode: "light".to_string(),
            theme_accent: "sunset".to_string(),
            celebration_style: "stars".to_string(),
        };

        save_settings(&path, settings).expect("settings should save");
        let loaded = load_settings(&path).expect("settings should load");

        assert!(!loaded.sound_enabled);
        assert_eq!(loaded.sound_volume, 0.4);
        assert_eq!(loaded.default_duration, 45);
        assert_eq!(loaded.theme_mode, "light");

        if let Some(parent) = path.parent() {
            std::fs::remove_dir_all(parent).ok();
        }
    }

    #[test]
    fn invalid_values_are_sanitized() {
        let path = temp_settings_path("sanitize");
        let settings = AppSettings {
            sound_enabled: true,
            sound_volume: 2.0,
            default_duration: -1,
            theme_mode: "unknown".to_string(),
            theme_accent: "unknown".to_string(),
            celebration_style: "unknown".to_string(),
        };

        let saved = save_settings(&path, settings).expect("settings should save");

        assert_eq!(saved.sound_volume, 1.0);
        assert_eq!(saved.default_duration, 25);
        assert_eq!(saved.theme_mode, "dark");
        assert_eq!(saved.theme_accent, "ocean");
        assert_eq!(saved.celebration_style, "confetti");

        if let Some(parent) = path.parent() {
            std::fs::remove_dir_all(parent).ok();
        }
    }
}
