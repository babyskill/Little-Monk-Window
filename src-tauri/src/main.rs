mod quote;
mod scheduler;
mod settings;
mod tray;
mod window;

use crate::quote::{DhammapadaVerse, QuoteLibrary, QuotePayload};
use crate::settings::AppSettings;
use std::sync::{Arc, RwLock};
use tauri::{AppHandle, Emitter, Manager, State};

#[derive(Clone)]
struct SharedSettings(Arc<RwLock<AppSettings>>);

#[derive(Clone)]
struct SharedQuotes(Arc<RwLock<QuoteLibrary>>);

#[tauri::command]
fn get_settings(settings: State<'_, SharedSettings>) -> AppSettings {
    settings
        .0
        .read()
        .map(|guard| guard.clone())
        .unwrap_or_default()
}

#[tauri::command]
fn update_settings(
    app: AppHandle,
    settings_state: State<'_, SharedSettings>,
    next: AppSettings,
) -> Result<AppSettings, String> {
    {
        let mut guard = settings_state
            .0
            .write()
            .map_err(|_| "failed to lock settings".to_string())?;
        *guard = next.clone();
    }

    settings::save_settings(&next).map_err(|err| err.to_string())?;
    window::set_always_on_top(&app, next.always_on_top);
    let _ = app.emit("settings:changed", next.clone());
    Ok(next)
}

#[tauri::command]
fn show_settings_window(app: AppHandle) {
    window::show_settings_window(&app);
}

#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn list_verses(quotes: State<'_, SharedQuotes>) -> Result<Vec<DhammapadaVerse>, String> {
    quotes
        .0
        .read()
        .map(|guard| guard.list())
        .map_err(|_| "failed to lock content library".to_string())
}

#[tauri::command]
fn current_quote(
    quotes: State<'_, SharedQuotes>,
    settings: State<'_, SharedSettings>,
) -> Result<QuotePayload, String> {
    let language = settings
        .0
        .read()
        .map(|guard| guard.quote_language.clone())
        .unwrap_or_else(|_| AppSettings::default().quote_language);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();

    quotes
        .0
        .read()
        .map(|guard| guard.quote_for_time(now, &language))
        .map_err(|_| "failed to lock content library".to_string())
}

#[tauri::command]
fn get_next_quote(
    quotes: State<'_, SharedQuotes>,
    settings: State<'_, SharedSettings>,
    current_id: String,
) -> Result<QuotePayload, String> {
    let language = settings
        .0
        .read()
        .map(|guard| guard.quote_language.clone())
        .unwrap_or_else(|_| AppSettings::default().quote_language);

    quotes
        .0
        .read()
        .map(|guard| guard.next_quote(&current_id, &language))
        .map_err(|_| "failed to lock content library".to_string())
}

#[tauri::command]
fn upsert_verse(
    app: AppHandle,
    quotes: State<'_, SharedQuotes>,
    verse: DhammapadaVerse,
) -> Result<DhammapadaVerse, String> {
    let saved = {
        let mut guard = quotes
            .0
            .write()
            .map_err(|_| "failed to lock content library".to_string())?;
        guard.upsert(verse).map_err(|err| err.to_string())?
    };

    let _ = app.emit("content:changed", ());
    Ok(saved)
}

#[tauri::command]
fn delete_verse(
    app: AppHandle,
    quotes: State<'_, SharedQuotes>,
    id: String,
) -> Result<bool, String> {
    let changed = {
        let mut guard = quotes
            .0
            .write()
            .map_err(|_| "failed to lock content library".to_string())?;
        guard.remove(&id).map_err(|err| err.to_string())?
    };

    if changed {
        let _ = app.emit("content:changed", ());
    }
    Ok(changed)
}

#[tauri::command]
fn reset_verses(
    app: AppHandle,
    quotes: State<'_, SharedQuotes>,
) -> Result<Vec<DhammapadaVerse>, String> {
    let verses = {
        let mut guard = quotes
            .0
            .write()
            .map_err(|_| "failed to lock content library".to_string())?;
        guard.reset_to_bundle().map_err(|err| err.to_string())?
    };

    let _ = app.emit("content:changed", ());
    Ok(verses)
}

#[tauri::command]
async fn update_verses_from_github(
    app: AppHandle,
    quotes: State<'_, SharedQuotes>,
    zip_url: String,
) -> Result<usize, String> {
    let quotes = quotes.0.clone();
    let count = tauri::async_runtime::spawn_blocking(move || {
        let mut guard = quotes
            .write()
            .map_err(|_| "failed to lock content library".to_string())?;
        guard
            .update_from_github_zip(&zip_url)
            .map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| err.to_string())??;

    let _ = app.emit("content:changed", ());
    Ok(count)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_settings,
            update_settings,
            show_settings_window,
            quit_app,
            list_verses,
            current_quote,
            get_next_quote,
            upsert_verse,
            delete_verse,
            reset_verses,
            update_verses_from_github
        ])
        .setup(|app| {
            let settings = Arc::new(RwLock::new(settings::load_settings()));
            let library = Arc::new(RwLock::new(QuoteLibrary::load()?));
            let app_handle = app.handle().clone();

            let current = settings
                .read()
                .map(|guard| guard.clone())
                .unwrap_or_default();
            let _ = settings::save_settings(&current);
            window::set_accessory_activation_policy(&app_handle);
            window::configure_main_window(&app_handle, current.always_on_top);
            window::configure_settings_window(&app_handle);
            tray::setup_tray(&app_handle)?;
            scheduler::start_quote_loop(app_handle.clone(), library.clone(), settings.clone());
            scheduler::start_bell_loop(app_handle.clone(), library.clone(), settings.clone());

            app.manage(SharedSettings(settings));
            app.manage(SharedQuotes(library));

            Ok(())
        })
        .on_menu_event(tray::handle_menu_event)
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                if window.label() == "settings" {
                    let app = window.app_handle();
                    let _ = window.hide();
                    window::set_accessory_activation_policy(&app);
                    return;
                }
                let _ = window.hide();
            }
        })
        .build(tauri::generate_context!())
        .expect("failed to build app")
        .run(|_, _| {});
}
