use std::{fs, path::PathBuf, sync::Arc};

use app_dirs2::{get_app_root, AppDataType};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use ts_rs::TS;
use uuid::{self, Uuid};

use super::plugins::theme::{theme_name, theme_uuid};

use crate::{js_manage, APP_INFO};

pub const CONFIG_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../bindings/")]
pub struct EDTRConfig {
    #[serde(rename = "disabledPlugins", default = "Default::default")]
    pub disabled_plugins: Vec<Uuid>,

    pub theme_uuid: Uuid,
    pub theme_ver: String,
}
js_manage!(crate::config::EDTRConfig);

impl Default for EDTRConfig {
    fn default() -> Self {
        Self {
            disabled_plugins: vec![],

            theme_uuid: theme_uuid(),
            theme_ver: theme_name(),
        }
    }
}

// creates a config file and returns a default config
pub fn create_return_default(cfg: PathBuf) -> EDTRConfig {
    let d = EDTRConfig::default();

    if let Err(e) = fs::write(cfg, serde_json::to_vec(&d).unwrap()) {
        edtr_log::warn!("failed to create new config! ({e})");
    };

    d
}

pub async fn load_config(app: Arc<AppHandle>) {
    let config_path = get_app_root(AppDataType::SharedData, &APP_INFO)
        .map_err(|_| {
            edtr_log::error!("root folder does not exist!");

            panic!();
        })
        .unwrap()
        .join(CONFIG_NAME);

    let config: EDTRConfig = match fs::read(config_path.clone()) {
        Err(_) => {
            edtr_log::debug!("no config found, creating default");

            create_return_default(config_path)
        }
        Ok(ref bytes) => match serde_json::from_slice(bytes) {
            Err(e) => {
                edtr_log::error!("failed to parse config.json! ({e})");

                create_return_default(config_path)
            }
            Ok(json) => json,
        },
    };

    app.manage(config);
}
