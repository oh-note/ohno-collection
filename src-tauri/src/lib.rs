// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use std::sync::Arc;
use std::thread;
mod server;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    builder
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle();
            tray::create_tray(handle)?;
            let handle_thread = Arc::new(handle.clone());

            thread::spawn(move || {
                server::init(handle_thread).unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
