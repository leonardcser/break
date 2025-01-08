use tauri::{
    menu::{Menu, MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
    Manager, WebviewUrl, WebviewWindowBuilder,
};

#[tauri::command]
fn timer_done(app: tauri::AppHandle) {
    if let Some(break_window) = app.get_webview_window("break") {
        break_window.hide().unwrap();
    }
}

fn open_break(app: &tauri::AppHandle) {
    // Check if window already exists
    if let Some(break_window) = app.get_webview_window("break") {
        break_window.maximize().unwrap();
        break_window.set_focus().unwrap();
        return;
    }

    // Create new window
    let ww = WebviewWindowBuilder::new(
        app,
        "break",                          // window label
        WebviewUrl::App("/break".into()), // URL
    )
    .title("Break Time")
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let _menu = MenuBuilder::new(app).copy().build()?;

            let info_i = MenuItem::with_id(app, "info", "Break v0.1.0", false, None::<&str>)?;
            let break_i = MenuItem::with_id(app, "break", "Break", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&info_i, &break_i, &quit_i])?;
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "break" => open_break(&app),
                    "quit" => {
                        app.exit(0);
                    }
                    _ => unreachable!(),
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![timer_done])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
