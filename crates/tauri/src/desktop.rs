use crate::models::*;
use crate::{Config, ProtocolConfig};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, Runtime};
use webview_bundle_protocol::{BundleProtocol, FileSource, LocalProtocol, Protocol};

pub fn init<R: Runtime>(app: &AppHandle<R>, config: Config) -> crate::Result<WebviewBundle<R>> {
  let mut webview_bundle = WebviewBundle::new(app.clone(), config);
  webview_bundle.init()?;
  Ok(webview_bundle)
}

/// Access to the tauri APIs.
pub struct WebviewBundle<R: Runtime> {
  app: AppHandle<R>,
  config: Config,
  protocols: HashMap<String, Arc<dyn Protocol>>,
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
      let protocol: Arc<dyn Protocol> = match protocol_config {
        ProtocolConfig::Bundle { dir, base_dir, .. } => {
          let base_dir = base_dir
            .clone()
            .and_then(|x| BaseDirectory::from_variable(&x))
            .unwrap_or(BaseDirectory::Resource);
          let source_dir = self
            .app
            .path()
            .resolve("", base_dir)?
            .join(dir.clone().unwrap_or_else(|| "bundles".to_string()));
          let source = FileSource::new(source_dir);
          Arc::new(BundleProtocol::new(source))
        }
        ProtocolConfig::Local { hosts, .. } => Arc::new(LocalProtocol::new(hosts.clone())),
      };
      if self.protocols.contains_key(&scheme) {
        return Err(crate::Error::ProtocolSchemeDuplicated { scheme });
      }
      self.protocols.insert(scheme, protocol);
    }
    Ok(())
  }

  pub(crate) fn get_protocol(&self, scheme: &str) -> Option<&Arc<dyn Protocol>> {
    self.protocols.get(scheme)
  }

  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
