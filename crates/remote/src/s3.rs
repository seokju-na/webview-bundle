use crate::common::{HttpConfig, OpendalConfig, OpendalRemote, EXTENSION, MIME_TYPE};
use crate::{Builder, Config, Remote};
use async_trait::async_trait;
use std::io::Cursor;
use webview_bundle::{Bundle, BundleReader, BundleWriter, Reader, Writer};

#[derive(Default)]
#[non_exhaustive]
pub struct AwsConfig {
  pub access_key_id: Option<String>,
  pub secret_access_key: Option<String>,
  pub session_token: Option<String>,
  pub bucket: String,
  pub region: Option<String>,
  pub endpoint: Option<String>,
  pub role_arn: Option<String>,
  pub role_session_name: Option<String>,
  pub external_id: Option<String>,
  pub opendal: OpendalConfig,
}

impl Config for AwsConfig {
  type Builder = AwsBuilder;

  fn into_builder(self) -> Self::Builder {
    AwsBuilder { config: self }
  }
}

#[derive(Default)]
pub struct AwsBuilder {
  config: AwsConfig,
}

impl AwsBuilder {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Builder for AwsBuilder {
  type Config = AwsConfig;

  fn build(self) -> crate::Result<impl Remote> {
    let mut s = opendal::services::S3::default().bucket(&self.config.bucket);
    if let Some(ref access_key_id) = self.config.access_key_id {
      s = s.access_key_id(access_key_id);
    }
    if let Some(ref secret_access_key) = self.config.secret_access_key {
      s = s.secret_access_key(secret_access_key);
    }
    if let Some(ref session_token) = self.config.session_token {
      s = s.session_token(session_token);
    }
    if let Some(ref region) = self.config.region {
      s = s.region(region);
    }
    if let Some(ref endpoint) = self.config.endpoint {
      s = s.endpoint(endpoint);
    }
    if let Some(ref role_arn) = self.config.role_arn {
      s = s.role_arn(role_arn);
    }
    if let Some(ref role_session_name) = self.config.role_session_name {
      s = s.role_session_name(role_session_name);
    }
    if let Some(ref external_id) = self.config.external_id {
      s = s.external_id(external_id);
    }
    // let remote = OpendalRemote::new(s, config);
    // Ok(Aws {
    //   config: self.config,
    //   op,
    // })
    todo!()
  }
}

pub struct Aws {
  config: AwsConfig,
  remote: OpendalRemote,
}

#[async_trait]
impl Remote for Aws {
  async fn upload(&self, bundle_name: &str, version: &str, bundle: &Bundle) -> crate::Result<()> {
    self.remote.upload(bundle_name, version, bundle).await
  }

  async fn download(&self, bundle_name: &str, version: &str) -> crate::Result<Bundle> {
    self.remote.download(bundle_name, version).await
  }
}
