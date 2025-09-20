use webview_bundle_tauri::{Config, Protocol};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {name}! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(webview_bundle_tauri::init(
      Config::new()
        .protocol(Protocol::bundle("bundle").dir("../../examples/tauri-simple/bundles"))
        .protocol(Protocol::local("local").host("example.com", "http://localhost:1420")),
    ))
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
