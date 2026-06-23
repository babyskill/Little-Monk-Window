use anyhow::Context;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub always_on_top: bool,
    pub show_quote: bool,
    pub show_idle_message: bool,
    pub show_tap_message: bool,
    pub quote_interval_secs: u64,
    pub quote_visible_secs: u64,
    pub pet_size: f64,
    pub font_size: f64,
    pub quote_language: String,
    pub bell_enabled: bool,
    pub bell_interval_minutes: u64,
    pub bell_sync_message: bool,
    pub bell_sound_enabled: bool,
    pub bell_sound: String,
    pub bell_volume: f64,
    pub bell_repeat_count: u8,
    pub quiet_hours_enabled: bool,
    pub quiet_hours_start_minutes: u16,
    pub quiet_hours_end_minutes: u16,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            always_on_top: true,
            show_quote: true,
            show_idle_message: true,
            show_tap_message: true,
            quote_interval_secs: 300,
            quote_visible_secs: 20,
            pet_size: 145.0,
            font_size: 17.0,
            quote_language: "vi".to_string(),
            bell_enabled: false,
            bell_interval_minutes: 30,
            bell_sync_message: true,
            bell_sound_enabled: true,
            bell_sound: "bell".to_string(),
            bell_volume: 0.75,
            bell_repeat_count: 1,
            quiet_hours_enabled: false,
            quiet_hours_start_minutes: 22 * 60,
            quiet_hours_end_minutes: 7 * 60,
        }
    }
}

pub fn app_config_dir() -> anyhow::Result<PathBuf> {
    let dirs = ProjectDirs::from("com", "company", "LittleMonkWindow")
        .context("failed to resolve app data directory")?;
    Ok(dirs.config_dir().to_path_buf())
}

fn settings_path() -> anyhow::Result<PathBuf> {
    Ok(app_config_dir()?.join("settings.json"))
}

pub fn load_settings() -> AppSettings {
    let path = match settings_path() {
        Ok(path) => path,
        Err(_) => return AppSettings::default(),
    };

    fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn save_settings(settings: &AppSettings) -> anyhow::Result<()> {
    let path = settings_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let data = serde_json::to_string_pretty(settings)?;
    fs::write(path, data)?;
    Ok(())
}
