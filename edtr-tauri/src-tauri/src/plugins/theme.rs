use std::{fs, path::PathBuf, sync::Arc};

use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::config::EDTRConfig;

use super::{EDTRPlugins, PluginType};

// default edtr theme info
pub const fn theme_uuid() -> Uuid {
    Uuid::from_bytes([
        0xE, 0xD, 0xF, 0xD, 0x7, 0x0, 0xD, 0x9, 0xE, 0xA, 0xB, 0xA, 0x5, 0x5, 0xC, 0x2,
    ])
}

pub fn theme_name() -> String {
    "edtr-dark".into()
}

pub type ThemeVersions = HashMap<String, PathBuf>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
enum ThemeType {
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ThemeColours {
    primary_dark: String,
}

// loaded using the data from the selected theme (`Theme` struct)
#[derive(Serialize, Deserialize, Debug)]
pub struct EDTRTheme {
    name: String,

    #[serde(rename = "type")]
    ttype: ThemeType,
    colours: ThemeColours,
}

// this shouldnt ever fail as the builtin themes are guaranted to exist and be correct
// but thats assuming no one has modified them
fn get_default_theme(plugins: &EDTRPlugins) -> EDTRTheme {
    edtr_log::warn!("loading default EDTR theme due to previous error!");

    let default_theme = plugins
        .find_by_uuid(theme_uuid())
        .unwrap_or_else(|| edtr_log::fatal!("builtin EDTR theme has incorrect UUID!"));

    #[allow(irrefutable_let_patterns)]
    let theme = if let PluginType::Theme { ref themes } = default_theme.ptype {
        // builtin edtr theme always exists
        themes.get(&theme_name()).unwrap_or_else(|| {
            edtr_log::fatal!("builtin EDTR theme has missing/incorrect variant!")
        })
    } else {
        edtr_log::fatal!("builtin EDTR theme does not have `\"type\":\"theme\"`!");
    };

    let theme_path = default_theme.path.join(theme);

    serde_json::from_slice(
        &fs::read(theme_path)
            .unwrap_or_else(|_| edtr_log::fatal!("failed to find builtin EDTR theme variation!")),
    )
    .unwrap_or_else(|_| edtr_log::fatal!("faield to parse builtin EDTR theme"))
}

// we want to load the themes separately from the other plugins for a couple reaons:
// - other plugins are loaded after the actual editor is loaded, and theme is needed before that
// - only 1 theme can be selected so we dont want to load them in a loop as it would not properly load the theme the user picked
pub async fn load_theme(app: Arc<AppHandle>) {
    let app_cfg = app.state::<EDTRConfig>();
    let plugins = app.state::<EDTRPlugins>();

    let plugin = plugins.find_by_uuid(app_cfg.theme_uuid);
    let theme_cfg = match plugin {
        Some(p) => p,
        None => {
            edtr_log::error!(
                "could not find theme with given uuid `{}`!",
                app_cfg.theme_uuid
            );

            app.manage(get_default_theme(&plugins));
            return;
        }
    };

    // TODO: if we fallback to default theme, is getting and managing better than handling every error?

    let theme = match theme_cfg.ptype {
        PluginType::Theme { ref themes } => match themes.get(&app_cfg.theme_ver) {
            Some(t) => t,
            None => {
                edtr_log::error!("no theme version named `{}` was found!", app_cfg.theme_ver);

                app.manage(get_default_theme(&plugins));
                return;
            }
        },
        #[allow(unreachable_patterns)]
        _ => {
            edtr_log::error!("plugin `{}` is not a theme!", theme_cfg.name);

            app.manage(get_default_theme(&plugins));
            return;
        }
    };

    let theme_path = theme_cfg.path.join(theme);

    let bytes = match fs::read(theme_path) {
        Ok(b) => b,
        Err(e) => {
            edtr_log::error!("theme variation ({theme:?}) does not exist!");

            app.manage(get_default_theme(&plugins));
            return;
        }
    };

    let edtr_theme: EDTRTheme = match serde_json::from_slice(&bytes) {
        Ok(t) => t,
        Err(e) => {
            edtr_log::error!("failed to theme definition for `{}` ({e})", theme_cfg.name);

            app.manage(get_default_theme(&plugins));
            return;
        }
    };

    edtr_log::debug!("loaded theme `{}`", theme_cfg.name);

    app.manage(edtr_theme);
}
