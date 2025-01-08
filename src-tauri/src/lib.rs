use tauri::menu::MenuBuilder;
// use window_vibrancy::*;

#[tauri::command]
fn timer_done(webview_window: tauri::WebviewWindow) {
    webview_window.close().expect("error while closing window");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // let window = app.get_webview_window("main").unwrap();
            //
            // #[cfg(target_os = "macos")]
            // apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
            //     .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            //
            // #[cfg(target_os = "windows")]
            // apply_blur(&window, Some((18, 18, 18, 125)))
            //     .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            let menu = MenuBuilder::new(app).copy().build()?;
            app.set_menu(menu)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![timer_done])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
