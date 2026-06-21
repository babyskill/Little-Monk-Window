use anyhow::Context;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub always_on_top: bool,
    pub show_quote: bool,
    pub quote_interval_secs: u64,
    pub quote_visible_secs: u64,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            always_on_top: true,
            show_quote: true,
            quote_interval_secs: 300,
            quote_visible_secs: 20,
        }
    }
}

fn settings_dir() -> anyhow::Result<PathBuf> {
    let dirs = ProjectDirs::from("com", "company", "LittleMonkWindow")
        .context("failed to resolve app data directory")?;
    Ok(dirs.config_dir().to_path_buf())
}

fn settings_path() -> anyhow::Result<PathBuf> {
    Ok(settings_dir()?.join("settings.json"))
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
