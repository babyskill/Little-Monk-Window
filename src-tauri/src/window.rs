use tauri::{AppHandle, Manager, WebviewWindow};

pub fn configure_main_window(app: &AppHandle, always_on_top: bool) {
    if let Some(window) = app.get_webview_window("pet") {
        toggle_click_through(&window, false);
        let _ = window.set_always_on_top(always_on_top);
        let _ = window.show();
    }
}

pub fn set_window_visible(app: &AppHandle, visible: bool) {
    if let Some(window) = app.get_webview_window("pet") {
        if visible {
            let _ = window.show();
            let _ = window.set_focus();
        } else {
            let _ = window.hide();
        }
    }
}

pub fn set_always_on_top(app: &AppHandle, always_on_top: bool) {
    if let Some(window) = app.get_webview_window("pet") {
        let _ = window.set_always_on_top(always_on_top);
    }
}

pub fn toggle_click_through(window: &WebviewWindow, ignore: bool) {
    let _ = window.set_ignore_cursor_events(ignore);
}
