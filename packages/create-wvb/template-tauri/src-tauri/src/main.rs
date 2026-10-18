#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use wvb_tauri::{Config, ProtocolConfig};

fn main() {
    tauri::Builder::default()
        .plugin(wvb_tauri::init(
            Config::new().protocol(ProtocolConfig::bundle("bundle")),
        ))
        .run(tauri::generate_context!())
        .expect("failed to run application");
}
