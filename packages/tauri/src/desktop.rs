use crate::{Config, Protocol};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Runtime};
use webview_bundle::protocol;
use webview_bundle::source::BundleSource;

pub fn init<R: Runtime>(
  app: &AppHandle<R>,
  config: Arc<Config<R>>,
) -> crate::Result<WebviewBundle<R>> {
  let webview_bundle =
    tauri::async_runtime::block_on(async move { WebviewBundle::init(app.clone(), config).await })?;
  Ok(webview_bundle)
}

/// Access to the tauri APIs.
pub struct WebviewBundle<R: Runtime> {
  _app: AppHandle<R>,
  _config: Arc<Config<R>>,
  _source: Arc<BundleSource>,
  protocols: HashMap<String, Arc<dyn protocol::Protocol>>,
}

impl<R: Runtime> WebviewBundle<R> {
  pub(crate) async fn init(app: AppHandle<R>, config: Arc<Config<R>>) -> crate::Result<Self> {
    let source = Arc::new(
      BundleSource::init(
        config.source.resolve_builtin_dir(&app)?.as_path(),
        config.source.resolve_remote_dir(&app)?.as_path(),
      )
      .await?,
    );
    let mut protocols = HashMap::with_capacity(config.protocols.len());
    for protocol_config in &config.protocols {
      let scheme = protocol_config.scheme().to_string();
      let protocol: Arc<dyn protocol::Protocol> = match protocol_config {
        Protocol::Bundle(_) => Arc::new(protocol::BundleProtocol::new(source.clone())),
        Protocol::Local(config) => Arc::new(protocol::LocalProtocol::new(config.hosts.clone())),
      };
      if protocols.contains_key(&scheme) {
        return Err(crate::Error::ProtocolSchemeDuplicated { scheme });
      }
      protocols.insert(scheme, protocol);
    }
    Ok(Self {
      _app: app,
      _config: config,
      _source: source,
      protocols,
    })
  }

  pub(crate) fn get_protocol(&self, scheme: &str) -> Option<&Arc<dyn protocol::Protocol>> {
    self.protocols.get(scheme)
  }
}
