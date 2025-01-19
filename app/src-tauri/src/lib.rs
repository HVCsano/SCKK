use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};
use tauri_plugin_opener::OpenerExt;

mod util;

#[tauri::command]
async fn update_done(app: AppHandle) {
    app.emit("setloadertext", "Konfiguráció betöltése").unwrap();
    util::config::setup_folders();
    let config = util::config::load_config();
    if config.is_none() {
        let loader = app.get_webview_window("loader").unwrap();
        let main = app.get_webview_window("main").unwrap();
        app.emit("setloadertext", "Konfiguráció nem létezik")
            .unwrap();
        loader.close().unwrap();
        main.show().unwrap();
        main.set_focus().unwrap();
        app.emit("setmainpage", "noconfig").unwrap();
    }
}

#[tauri::command]
async fn start_app(app_handle: AppHandle) {
    let overlay = app_handle.get_webview_window("overlay").unwrap();
    let loader = app_handle.get_webview_window("loader").unwrap();
    loader.hide().unwrap();
    overlay.show().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {
            println!("Single instance callback");
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Kilépés", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            TrayIconBuilder::new()
                .menu(&menu)
                .title("SAMT App")
                .tooltip("SAMT App")
                .icon(app.default_window_icon().unwrap().clone())
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![update_done])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
