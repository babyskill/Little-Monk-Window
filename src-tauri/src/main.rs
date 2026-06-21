mod quote;
mod scheduler;
mod settings;
mod tray;
mod window;

use crate::quote::QuoteLibrary;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let settings = settings::load_settings();
            let library = QuoteLibrary::load_from_bundle()?;
            let app_handle = app.handle().clone();

            let _ = settings::save_settings(&settings);
            window::configure_main_window(&app_handle, settings.always_on_top);
            tray::setup_tray(&app_handle)?;
            scheduler::start_quote_loop(app_handle.clone(), library, settings.clone());

            app.manage(settings);

            Ok(())
        })
        .on_menu_event(tray::handle_menu_event)
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .build(tauri::generate_context!())
        .expect("failed to build app")
        .run(|_, _| {});
}
