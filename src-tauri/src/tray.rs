use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuEvent, MenuItem, SubmenuBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

use crate::window;

pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Show Monk", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide Monk", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
    let toggle_top = MenuItem::with_id(
        app,
        "toggle_top",
        "Toggle Always On Top",
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let submenu = SubmenuBuilder::new(app, "Little Monk")
        .items(&[&show, &hide, &settings, &toggle_top, &quit])
        .build()?;

    let menu = MenuBuilder::new(app).item(&submenu).build()?;
    let icon: Image<'static> = tauri::include_image!("icons/icon.png");

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .tooltip("Little Monk Window")
        .show_menu_on_left_click(false)
        .build(app)?;

    Ok(())
}

pub fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id().0.as_str() {
        "show" => window::set_window_visible(app, true),
        "hide" => window::set_window_visible(app, false),
        "settings" => window::show_settings_window(app),
        "toggle_top" => {
            if let Some(window) = app.get_webview_window("pet") {
                let current = window.is_always_on_top().unwrap_or(true);
                window::set_always_on_top(app, !current);
                let _ = window.set_focus();
            }
        }
        "quit" => app.exit(0),
        _ => {}
    }
}
