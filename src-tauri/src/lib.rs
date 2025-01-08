mod tray;

use std::time::Duration;
use tauri::{menu::MenuBuilder, Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
fn timer_done(app: tauri::AppHandle) {
    if let Some(break_window) = app.get_webview_window("break") {
        break_window.hide().unwrap();
    }
}

fn open_break(app: &tauri::AppHandle) {
    // Check if window already exists
    if let Some(mut break_window) = app.get_webview_window("break") {
        break_window.show().unwrap();
        break_window.set_focus().unwrap();
        break_window.navigate(break_window.url().unwrap()).unwrap();
        return;
    }

    // Create new window
    let ww = WebviewWindowBuilder::new(app, "break", WebviewUrl::App("/break".into()))
        .title("Break")
        .inner_size(1200.0, 800.0)
        .center()
        .minimizable(false)
        .maximizable(false)
        .decorations(false)
        .shadow(false)
        .hidden_title(true)
        .always_on_top(true)
        .closable(false)
        .transparent(true)
        .disable_drag_drop_handler()
        .incognito(true)
        .visible_on_all_workspaces(true)
        .maximized(true)
        .resizable(false)
        .devtools(false)
        .build()
        .unwrap();

    ww.set_focus().unwrap();
}

fn start_break_timer(app_handle: tauri::AppHandle) {
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(20 * 60));
        open_break(&app_handle);
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let _menu = MenuBuilder::new(app).copy().build()?;

            tray::create(app.app_handle())
                .unwrap()
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "break" => open_break(&app),
                    "quit" => {
                        app.exit(0);
                    }
                    _ => unreachable!(),
                });

            let app_handle = app.handle();
            start_break_timer(app_handle.clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![timer_done])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
