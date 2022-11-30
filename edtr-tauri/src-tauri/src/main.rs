#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cli;
mod config;
mod macros;
mod plugins;

use std::collections::HashMap;
use std::sync::Arc;

use app_dirs2::AppInfo;

use clap::Parser;
use plugins::theme::load_theme;
use tauri::async_runtime;
use tauri::{AppHandle, Manager};
use tauri_plugin_log::fern::colors::{Color, ColoredLevelConfig};
use tauri_plugin_log::{LogTarget, LoggerBuilder};
use window_shadows::set_shadow;
#[allow(unused_imports)]
use window_vibrancy::{
    apply_acrylic, apply_blur, apply_mica, apply_vibrancy, NSVisualEffectMaterial,
};

use crate::cli::CliArgs;
use crate::config::load_config;
use crate::plugins::load_plugins;

pub const APP_INFO: AppInfo = AppInfo {
    name: "EDTR",
    author: "EDTR",
};

// #[tauri::command]
// pub(crate) fn get_cli_args(args: State<CliArgs>) -> &CliArgs {
//     args.inner()
// }

fn main() {
    #[cfg(target_os = "linux")]
    compile_error!("edtr is not supported on linux");

    let log_colours = ColoredLevelConfig::default()
        .debug(Color::Cyan)
        .info(Color::Green);

    tauri::Builder::default()
        //.invoke_handler(tauri::generate_handler![get_cli_args])
        .setup(|app| {
            let app = Arc::new(app.app_handle());

            let args = CliArgs::parse_from(app.env().args);

            if args.devtools {
                app.get_window("main").unwrap().open_devtools();
            }

            app.manage(args);

            let window = app.get_window("edtr").unwrap();

            // window shadows
            set_shadow(&window, true).unwrap();

            // background blur
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(12)).unwrap();

            #[cfg(target_os = "windows")]
            // apply_mica(&window);
            apply_blur(&window, Some((21, 21, 21, 200))).unwrap();

            // https://github.com/tauri-apps/tao/issues/72
            // even with decorations off, this still occurs if the window is maximised in the config
            // so we have to manually maximise instead
            window.maximize().unwrap();

            let splash = app.get_window("splash").unwrap();

            // wait for splash content to be visible
            splash.once("splash:visible", move |_| {
                async_runtime::spawn(setup(Arc::clone(&app)));
            });

            Ok(())
        })
        .plugin(
            LoggerBuilder::new()
                .targets([LogTarget::Stdout, LogTarget::Webview, LogTarget::LogDir])
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}] [{}] [{}]: {}",
                        time::OffsetDateTime::now_utc()
                            .format(
                                &time::format_description::parse("[hour]:[minute]:[second]")
                                    .unwrap()
                            )
                            .unwrap(),
                        // get the key "from" (passed in by `src/utils/log.ts` + `edtr-log`)
                        record.key_values().get("from".into()).map_or_else(
                            // other modules may log which wont have the `from` key so we take their module name instead
                            || record.module_path().unwrap_or("<unknown>").into(),
                            |o| o.to_string()
                        ),
                        log_colours.color(record.level()),
                        message
                    ))
                })
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn setup(app: Arc<AppHandle>) {
    //static mut COUNTS: f32 = 0.0;
    //to_setup!(window | load_plugins);

    // async_runtime::spawn({
    //     let w = window.clone();

    //     async move {
    //         test().await;
    //         w.emit(
    //             "splash:progress",
    //             (unsafe {
    //                 COUNTS += 1.0;
    //                 COUNTS
    //             } / 2.0)
    //                 * 100.0,
    //         )
    //         .unwrap();
    //     }
    // });

    // async_runtime::spawn({
    //     let w = window;

    //     async move {
    //         test2().await;
    //         w.emit(
    //             "splash:progress",
    //             (unsafe {
    //                 COUNTS += 1.0;
    //                 COUNTS
    //             } / 2.0)
    //                 * 100.0,
    //         )
    //         .unwrap();
    //     }
    // });

    load_config(Arc::clone(&app)).await;

    load_plugins(Arc::clone(&app)).await;

    load_theme(Arc::clone(&app)).await;

    println!(
        "current theme: {:?}",
        app.state::<plugins::theme::EDTRTheme>()
    );

    app.get_window("splash")
        .unwrap()
        .emit("splash:loaded", 0.0)
        .unwrap();

    app.get_window("edtr").unwrap().show().unwrap();
}

pub struct Object {
    object_id: u32,
    pos: (f64, f64),
    rot: f64,
    scale: f64,
    props: HashMap<u16, String>,
}

pub struct EDTRState {
    // objects
    objects: Vec<Object>,
}
