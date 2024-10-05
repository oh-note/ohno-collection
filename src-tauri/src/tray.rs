use tauri::{tray::TrayIconBuilder, Runtime};

use tauri::menu::{Menu, MenuItem};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;
    let _ = TrayIconBuilder::with_id("tray")
        .tooltip("tauri")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .build(app);
    Ok(())
}
