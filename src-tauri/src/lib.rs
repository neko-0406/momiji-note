use tauri::Manager;

use crate::setting::AppSetting;

mod setting;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| -> Result<(), Box<dyn std::error::Error>> {
            let setting = match AppSetting::init(&app) {
                Ok(setting) => setting,
                Err(_) => panic!("設定の初期化に失敗しました"),
            };

            app.manage(setting);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
