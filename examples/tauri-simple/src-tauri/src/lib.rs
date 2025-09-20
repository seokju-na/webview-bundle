use std::collections::HashMap;
use webview_bundle_tauri::{Config, ProtocolConfig};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {name}! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let mut hosts = HashMap::new();
  hosts.insert(
    "example.com".to_string(),
    "http://localhost:1420".to_string(),
  );
  tauri::Builder::default()
    .plugin(webview_bundle_tauri::init(Config {
      protocols: vec![
        ProtocolConfig::Local {
          scheme: "local".to_string(),
          hosts,
        },
        ProtocolConfig::Bundle {
          scheme: "bundle".to_string(),
          dir: Some("../../examples/tauri-simple/bundles".to_string()),
          base_dir: None,
        },
      ],
    }))
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
