use tauri::{
  http,
  plugin::{Builder, TauriPlugin},
  Manager, Runtime, UriSchemeContext,
};

pub use config::*;
pub use models::*;

#[cfg(desktop)]
mod desktop;

mod commands;
mod config;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::WebviewBundle;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the tauri APIs.
pub trait WebviewBundleExtra<R: Runtime> {
  fn webview_bundle(&self) -> &WebviewBundle<R>;
}

impl<R: Runtime, T: Manager<R>> WebviewBundleExtra<R> for T {
  fn webview_bundle(&self) -> &WebviewBundle<R> {
    self.state::<WebviewBundle<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>(config: Config) -> TauriPlugin<R, Option<JsonConfig>> {
  let c = config.clone();
  let mut builder = Builder::<R, Option<JsonConfig>>::new("webview-bundle")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(move |app, _api| {
      #[cfg(desktop)]
      let webview_bundle = desktop::init(app, c)?;
      app.manage(webview_bundle);
      Ok(())
    });
  for protocol_config in config.protocols {
    let scheme = protocol_config.scheme().to_string();
    builder = builder.register_asynchronous_uri_scheme_protocol(
      protocol_config.scheme(),
      move |ctx: UriSchemeContext<R>, req, res| {
        let protocol = ctx
          .app_handle()
          .webview_bundle()
          .get_protocol(&scheme)
          .unwrap_or_else(|| panic!("protocol not found: {scheme}"))
          .clone();
        tauri::async_runtime::spawn(async move {
          match protocol.handle(req).await {
            Ok(resp) => res.respond(resp),
            Err(e) => {
              let resp = http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body(
                  format!("webview bundle protocol error: {e}")
                    .as_bytes()
                    .to_vec(),
                )
                .unwrap();
              res.respond(resp);
            }
          }
        });
      },
    )
  }
  builder.build()
}
