use crate::{setting::AppSetting, state::AppState};

#[tauri::command]
pub fn get_app_setting(state: tauri::State<'_, AppState>) -> Result<AppSetting, String> {
    let setting = state.setting.lock().unwrap().clone();
    Ok(setting)
}
