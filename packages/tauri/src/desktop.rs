use crate::{Config, Protocol};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Runtime};
use webview_bundle::{protocol, source};

pub fn init<R: Runtime>(app: &AppHandle<R>, config: Config) -> crate::Result<WebviewBundle<R>> {
  let mut webview_bundle = WebviewBundle::new(app.clone(), config);
  webview_bundle.init()?;
  Ok(webview_bundle)
}

/// Access to the tauri APIs.
pub struct WebviewBundle<R: Runtime> {
  app: AppHandle<R>,
  config: Config,
  protocols: HashMap<String, Arc<dyn protocol::Protocol>>,
}

impl<R: Runtime> WebviewBundle<R> {
  pub(crate) fn new(app: AppHandle<R>, config: Config) -> Self {
    let protocols_len = config.protocols.len();
    Self {
      app,
      config,
      protocols: HashMap::with_capacity(protocols_len),
    }
  }

  pub(crate) fn init(&mut self) -> crate::Result<()> {
    for protocol_config in &self.config.protocols {
      let scheme = protocol_config.scheme().to_string();
      let protocol: Arc<dyn protocol::Protocol> = match protocol_config {
        Protocol::Bundle(config) => {
          let source = source::FileSource::new(config.resolve_source_dir(&self.app)?);
          Arc::new(protocol::BundleProtocol::new(source))
        }
        Protocol::Local(config) => Arc::new(protocol::LocalProtocol::new(config.hosts.clone())),
      };
      if self.protocols.contains_key(&scheme) {
        return Err(crate::Error::ProtocolSchemeDuplicated { scheme });
      }
      self.protocols.insert(scheme, protocol);
    }
    Ok(())
  }

  pub(crate) fn get_protocol(&self, scheme: &str) -> Option<&Arc<dyn protocol::Protocol>> {
    self.protocols.get(scheme)
  }
}
