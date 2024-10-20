#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewUrl};
use webview_bundle_tauri::{Config, FSLoader};

fn main() {
  tauri::Builder::default()
    .plugin(webview_bundle_tauri::init("app", |app, _api| {
      let mut dir = app.path().resource_dir()?;
      dir.pop();
      dir.pop();
      dir.push("examples/tauri-simple");
      let config = Config::builder().loader(FSLoader::from_dir(dir)).build();
      Ok(config)
    }))
    .setup(|app| {
      let window = tauri::window::WindowBuilder::new(app, "primary").build()?;
      let webview_builder = tauri::webview::WebviewBuilder::new(
        "primary",
        WebviewUrl::CustomProtocol(url::Url::parse("app://bundle").unwrap()),
      );
      let _webview = window.add_child(
        webview_builder,
        tauri::LogicalPosition::new(0, 0),
        window.inner_size().unwrap(),
      );
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
