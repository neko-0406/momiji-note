use std::sync::Mutex;

use crate::setting::AppSetting;

#[derive(Debug)]
pub struct AppState {
    pub setting: Mutex<AppSetting>,
}
