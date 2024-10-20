mod config;
mod error;
mod loader;

pub use config::Config;
pub use error::Error;
pub use loader::{FSLoader, Loader};
use std::path::Path;
use tauri::http::{Method, Response, Uri};
use tauri::plugin::{PluginApi, TauriPlugin};
use tauri::{plugin, AppHandle, Manager, Runtime};

pub fn init<R, L, F>(scheme: &'static str, config: F) -> TauriPlugin<R>
where
  R: Runtime,
  L: Loader + Send + Sync + 'static,
  F: FnOnce(&AppHandle<R>, PluginApi<R, ()>) -> Result<Config<L>, Box<dyn std::error::Error>>
    + Send
    + 'static,
{
  plugin::Builder::<R>::new("webview-bundle")
    .setup(|app, api| {
      let config = config(app, api)?;
      app.manage(config);
      Ok(())
    })
    .register_asynchronous_uri_scheme_protocol(scheme, move |ctx, request, responder| {
      let method = request.method();
      if method != Method::GET {
        responder.respond(Response::builder().status(405).body(vec![]).unwrap());
        return;
      }
      let uri = request.uri().clone();
      let app = ctx.app_handle().clone();
      tauri::async_runtime::spawn(async move {
        let config = app.state::<Config<L>>();
        let bundle = config.loader().load(&uri).await.unwrap();
        let filepath = uri_to_filepath(&uri);
        let buf = bundle.read_file(filepath).unwrap();
        responder.respond(
          Response::builder()
            .header("content-type", "text/html")
            .header("content-length", buf.len())
            .status(200)
            .body(buf)
            .unwrap(),
        );
      });
    })
    .build()
}

fn uri_to_filepath(uri: &Uri) -> String {
  let filepath = uri.path()[1..].to_string();
  if Path::new(&filepath).extension().is_some() {
    return filepath;
  }
  let index_html = "index.html".to_string();
  if filepath.is_empty() {
    return index_html;
  }
  [filepath, index_html].join("/")
}
