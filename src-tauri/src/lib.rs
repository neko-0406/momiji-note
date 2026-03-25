mod setting;
mod state;
mod tauri_commands;

use tauri::Manager;

use crate::{setting::AppSetting, state::AppState};
use tauri_commands::get_app_setting;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| -> Result<(), Box<dyn std::error::Error>> {
            let setting = match AppSetting::init(&app) {
                Ok(setting) => setting,
                Err(_) => panic!("設定の初期化に失敗しました"),
            };

            let app_state = AppState { setting: setting };

            app.manage(app_state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_app_setting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
