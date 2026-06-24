use tauri::{AppHandle, Manager, WebviewWindow};

#[cfg(target_os = "macos")]
pub fn set_accessory_activation_policy(app: &AppHandle) {
    let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);
}

#[cfg(not(target_os = "macos"))]
pub fn set_accessory_activation_policy(_app: &AppHandle) {}

#[cfg(target_os = "macos")]
pub fn set_regular_activation_policy(app: &AppHandle) {
    let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
}

#[cfg(not(target_os = "macos"))]
pub fn set_regular_activation_policy(_app: &AppHandle) {}

pub fn configure_main_window(app: &AppHandle, always_on_top: bool) {
    if let Some(window) = app.get_webview_window("pet") {
        toggle_click_through(&window, false);
        let _ = window.set_always_on_top(always_on_top);
        let _ = window.center();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn configure_settings_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.set_always_on_top(false);
        let _ = window.hide();
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

pub fn show_settings_window(app: &AppHandle) {
    set_regular_activation_policy(app);
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
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
