// intermediary crate for logging on either wasm (using tauri-plugin-log-api) or non-wasm (log)
// wasm bindings assume `attachConsole` has already been called in the app somewhere.

#[cfg(target_arch = "wasm32")]
use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
#[derive(Serialize)]
pub struct LogOptions {
    pub from: &'static str,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "tauri-plugin-log-api")]
extern "C" {
    pub async fn debug(message: String, opts: JsValue);
    pub async fn info(message: String, opts: JsValue);
    pub async fn warn(message: String, opts: JsValue);
    pub async fn error(message: String, opts: JsValue);
}

#[macro_export]
macro_rules! debug {
    ($($args:tt)+) => {{
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async {
            edtr_log::debug(
                format!($($args)+),
                serde_wasm_bindgen::to_value(&edtr_log::LogOptions { from: env!("CARGO_PKG_NAME") }).unwrap()
            ).await
        });

        #[cfg(not(target_arch = "wasm32"))]
        log::debug!(from = env!("CARGO_PKG_NAME"); $($args)+);
    }};
}

#[macro_export]
macro_rules! info {
    ($($args:tt)+) => {{
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async {
            edtr_log::info(
                format!($($args)+),
                serde_wasm_bindgen::to_value(&edtr_log::LogOptions { from: env!("CARGO_PKG_NAME") }).unwrap()
            ).await
        });

        #[cfg(not(target_arch = "wasm32"))]
        log::info!(from = env!("CARGO_PKG_NAME"); $($args)+);
    }};
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)+) => {{
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async {
            edtr_log::warn(
                format!($($args)+),
                serde_wasm_bindgen::to_value(&edtr_log::LogOptions { from: env!("CARGO_PKG_NAME") }).unwrap()
            ).await
        });

        #[cfg(not(target_arch = "wasm32"))]
        log::warn!(from = env!("CARGO_PKG_NAME"); $($args)+);
    }};
}

#[macro_export]
macro_rules! error {
    ($($args:tt)+) => {{
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async {
            edtr_log::error(
                format!($($args)+),
                serde_wasm_bindgen::to_value(&edtr_log::LogOptions { from: env!("CARGO_PKG_NAME") }).unwrap()
            ).await
        });

        #[cfg(not(target_arch = "wasm32"))]
        log::error!(from = env!("CARGO_PKG_NAME"); $($args)+);
    }};
}

#[macro_export]
macro_rules! fatal {
    ($($args:tt)+) => {{
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async {
            edtr_log::error(
                format!("[FATAL] {}", format!($($args)+)),
                serde_wasm_bindgen::to_value(&edtr_log::LogOptions { from: env!("CARGO_PKG_NAME") }).unwrap()
            ).await
        });

        #[cfg(not(target_arch = "wasm32"))]
        log::error!(from = env!("CARGO_PKG_NAME"); "[FATAL] {}", format!($($args)+));

        panic!()
    }};
}
