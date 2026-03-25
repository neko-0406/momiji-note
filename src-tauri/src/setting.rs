use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::sync::Mutex;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tauri::App;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSetting {
    pub theme: String,
}

impl Default for AppSetting {
    fn default() -> Self {
        Self {
            theme: String::from("light"),
        }
    }
}

impl AppSetting {
    pub fn init(app: &App) -> Result<Mutex<AppSetting>, Box<dyn std::error::Error>> {
        let data_dir = app
            .path()
            .app_data_dir()
            .context("データディレクトリの取得に失敗しました")?;
        if !data_dir.exists() {
            fs::create_dir(&data_dir).context("データディレクトリの作成に失敗しました")?;
        }

        let config_file = data_dir.join("app_config.json");
        if !config_file.exists() {
            let file = File::create(&config_file).context("設定ファイルの作成に失敗しました")?;
            let setting = AppSetting::default();

            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &setting)
                .context("設定ファイルへテンプレートの書き込みに失敗しました")?
        }

        let setting = {
            let file = File::open(&config_file).context("設定ファイルの取得に失敗しました")?;
            let reader = BufReader::new(&file);
            let setting: AppSetting = match serde_json::from_reader(reader) {
                Ok(setting) => setting,
                Err(error) => {
                    println!(
                        "設定ファイルの読み取りに失敗しました: {:#?}",
                        error.to_string()
                    );
                    AppSetting::default()
                }
            };
            setting
        };
        Ok(Mutex::new(setting))
    }
}
