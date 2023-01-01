pub(crate) mod theme;

#[cfg(not(debug_assertions))]
use std::fs::{read, read_dir};

use std::collections::HashMap;
use std::sync::Arc;
use std::{fs, path::PathBuf};

#[cfg(not(debug_assertions))]
use app_dirs2::{get_app_dir, AppDataType};
use ts_rs::TS;
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::config::{EDTRConfig, CONFIG_NAME};
use crate::js_manage;
#[cfg(not(debug_assertions))]
use crate::APP_INFO;

pub fn plugins_folder() -> PathBuf {
    #[cfg(not(debug_assertions))]
    return get_app_dir(AppDataType::SharedData, &APP_INFO, "plugins/").map_err(|e| {
        edtr_log::fatal!("plugin dir does not exist! ({e})");
    });

    // cwd in debug is `/src-tauri/`
    #[cfg(debug_assertions)]
    return "./plugins/".into();
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../bindings/")]
pub struct EDTRPlugins(Vec<PluginConfig>);
js_manage!(crate::plugins::EDTRPlugins);

impl EDTRPlugins {
    pub fn new() -> EDTRPlugins {
        Self(vec![])
    }

    pub fn add(&mut self, plugin: PluginConfig) {
        self.0.push(plugin);
    }

    pub fn find_by_uuid(&self, uuid: Uuid) -> Option<&PluginConfig> {
        self.0.iter().find(|p| p.uuid == uuid)
    }

    pub fn plugins(&self) -> &Vec<PluginConfig> {
        &self.0
    }
}

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
#[ts(export, export_to = "../bindings/", rename_all = "lowercase")]
pub enum PluginType {
    Theme { themes: HashMap<String, PathBuf> },
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../bindings/")]
pub struct PluginConfig {
    name: String,
    description: Option<String>,
    version: String,
    author: Option<String>,
    repository: Option<String>,

    uuid: Uuid,

    r#type: PluginType,

    #[serde(skip)]
    pub path: PathBuf,
}

pub async fn load_plugins(app: Arc<AppHandle>) {
    let app_cfg = app.state::<EDTRConfig>();

    let plugins = fs::read_dir(plugins_folder()).unwrap();

    let mut fplugins = EDTRPlugins::new();

    edtr_log::debug!("found plugins");

    for plugin in plugins {
        match plugin {
            Ok(p) => {
                let config_path = p.path().join(CONFIG_NAME);

                let json = match fs::read(config_path) {
                    Ok(j) => j,
                    Err(_) => {
                        edtr_log::error!("plugin `{:?}` has no config.json!", p.file_name());
                        continue;
                    }
                };

                let mut config: PluginConfig = match serde_json::from_slice(&json) {
                    Ok(c) => c,
                    Err(e) => {
                        edtr_log::error!(
                            "failed to parse `{:?}` config.json! ({e})",
                            p.file_name()
                        );
                        continue;
                    }
                };
                config.path = p.path();

                if app_cfg.disabled_plugins.contains(&config.uuid) {
                    edtr_log::debug!("plugin `{}` is disabled", config.name);
                    continue;
                }

                fplugins.add(config);
            }
            Err(_) => edtr_log::warn!("non-plugin found in plugin directory: ({plugin:?})"),
        }
    }

    app.manage(fplugins);
}

// pub async fn run_plugins(app: Arc<AppHandle>) {
//     let app_cfg = app.state::<EDTRConfig>();
//     let plugins = app.state::<EDTRPlugins>();

//     for plugin in plugins.plugins() {
//         match plugin.ptype {
//             _ => continue,
//         }
//     }
// }
