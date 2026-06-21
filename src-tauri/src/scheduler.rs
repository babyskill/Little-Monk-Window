use crate::{quote::QuoteLibrary, settings::AppSettings};
use std::{thread, time::Duration};
use tauri::{AppHandle, Emitter};

pub fn start_quote_loop(app: AppHandle, library: QuoteLibrary, settings: AppSettings) {
    thread::spawn(move || {
        let visible = Duration::from_secs(settings.quote_visible_secs.max(1));
        let interval = Duration::from_secs(settings.quote_interval_secs.max(visible.as_secs() + 1));
        let hidden = interval.saturating_sub(visible);

        loop {
            let now = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(_) => 0,
            };
            let payload = library.quote_for_time(now);

            let _ = app.emit("monk:quote", payload);
            thread::sleep(visible);
            let _ = app.emit("monk:clear", ());
            thread::sleep(hidden);
        }
    });
}
