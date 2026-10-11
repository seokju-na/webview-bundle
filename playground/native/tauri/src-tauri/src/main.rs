#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use wvb_tauri::{Config, ProtocolConfig, RemoteConfig, UpdaterConfig};

fn main() {
  let remote: serde_json::Value = serde_json::from_str(include_str!("../../remote.json"))
    .expect("Cannot load remote config. make sure call \"yarn setup\" first.");
  tauri::Builder::default()
    .plugin(wvb_tauri::init(
      Config::new()
        .protocol(ProtocolConfig::bundle("bundle"))
        .remote(RemoteConfig::new(
          remote["baseUrl"].as_str().expect("baseUrl"),
        ))
        .updater(UpdaterConfig::new()),
    ))
    .run(tauri::generate_context!())
    .expect("failed to run playground");
}
