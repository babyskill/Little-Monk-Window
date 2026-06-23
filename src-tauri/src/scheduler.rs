use crate::{quote::QuoteLibrary, settings::AppSettings};
use chrono::Timelike;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};
use tauri::{AppHandle, Emitter};

pub fn start_quote_loop(
    app: AppHandle,
    library: Arc<RwLock<QuoteLibrary>>,
    settings: Arc<RwLock<AppSettings>>,
) {
    thread::spawn(move || loop {
        let current_settings = settings
            .read()
            .map(|guard| guard.clone())
            .unwrap_or_default();
        let visible = Duration::from_secs(current_settings.quote_visible_secs.max(1));
        let interval = Duration::from_secs(
            current_settings
                .quote_interval_secs
                .max(visible.as_secs() + 1),
        );
        let hidden = interval.saturating_sub(visible);
        let now = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0,
        };
        let payload = library
            .read()
            .map(|guard| guard.quote_for_time(now, &current_settings.quote_language));

        if let Ok(payload) = payload {
            let _ = app.emit("monk:quote", payload);
        }
        thread::sleep(visible);
        let _ = app.emit("monk:clear", ());
        thread::sleep(hidden);
    });
}

pub fn start_bell_loop(
    app: AppHandle,
    library: Arc<RwLock<QuoteLibrary>>,
    settings: Arc<RwLock<AppSettings>>,
) {
    thread::spawn(move || loop {
        let current_settings = settings
            .read()
            .map(|guard| guard.clone())
            .unwrap_or_default();

        if !current_settings.bell_enabled {
            thread::sleep(Duration::from_secs(5));
            continue;
        }

        if !is_quiet_time(&current_settings) {
            if current_settings.bell_sync_message {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|duration| duration.as_secs())
                    .unwrap_or_default();
                let payload = library
                    .read()
                    .map(|guard| guard.quote_for_time(now, &current_settings.quote_language));
                if let Ok(payload) = payload {
                    let _ = app.emit("monk:quote", payload);
                }
            }
            let _ = app.emit("monk:bell", ());
        }

        thread::sleep(Duration::from_secs(
            current_settings.bell_interval_minutes.max(1) * 60,
        ));
    });
}

fn is_quiet_time(settings: &AppSettings) -> bool {
    if !settings.quiet_hours_enabled {
        return false;
    }

    let now = chrono::Local::now();
    let current = (now.hour() * 60 + now.minute()) as u16;
    let start = settings.quiet_hours_start_minutes.min(1439);
    let end = settings.quiet_hours_end_minutes.min(1439);

    if start == end {
        return true;
    }
    if start < end {
        current >= start && current < end
    } else {
        current >= start || current < end
    }
}
