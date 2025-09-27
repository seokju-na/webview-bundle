use crate::{impl_opendal_config_for_builder, Builder, Config, HttpConfig, Remote};
use async_trait::async_trait;
use std::sync::Arc;
use webview_bundle::Bundle;

#[derive(Default, Clone)]
#[non_exhaustive]
pub struct VercelConfig {
  pub token: String,
  pub(crate) opendal: crate::common::opendal::OpendalConfig,
}

impl Config for VercelConfig {
  type Builder = VercelBuilder;

  fn into_builder(self) -> Self::Builder {
    VercelBuilder { config: self }
  }
}

#[derive(Default)]
pub struct VercelBuilder {
  config: VercelConfig,
}

impl VercelBuilder {
  pub fn token(mut self, token: impl Into<String>) -> Self {
    self.config.token = token.into();
    self
  }

  fn build_service(&self) -> opendal::services::VercelBlob {
    opendal::services::VercelBlob::default().token(&self.config.token)
  }
}

impl_opendal_config_for_builder!(VercelBuilder);

impl Builder for VercelBuilder {
  type Config = VercelConfig;

  fn build(self) -> crate::Result<impl Remote> {
    let service = self.build_service();
    let remote = crate::common::opendal::OpendalRemote::new(service, self.config.opendal)?;
    Ok(Vercel { remote })
  }
}

pub struct Vercel {
  remote: crate::common::opendal::OpendalRemote,
}

impl Vercel {
  pub fn builder() -> VercelBuilder {
    VercelBuilder::default()
  }
}

#[async_trait]
impl Remote for Vercel {
  async fn upload(&self, bundle_name: &str, version: &str, bundle: &Bundle) -> crate::Result<()> {
    self.remote.upload(bundle_name, version, bundle).await
  }

  async fn download(&self, bundle_name: &str, version: &str) -> crate::Result<Bundle> {
    self.remote.download(bundle_name, version).await
  }
}
