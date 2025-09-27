use crate::common::HttpConfig;
use crate::{impl_opendal_config_for_builder, Builder, Config, Remote};
use async_trait::async_trait;
use std::sync::Arc;
use webview_bundle::Bundle;

#[derive(Default, Clone)]
#[non_exhaustive]
pub struct S3Config {
  pub access_key_id: Option<String>,
  pub secret_access_key: Option<String>,
  pub session_token: Option<String>,
  pub bucket: String,
  pub region: Option<String>,
  pub endpoint: Option<String>,
  pub role_arn: Option<String>,
  pub role_session_name: Option<String>,
  pub external_id: Option<String>,
  pub(crate) opendal: crate::common::opendal::OpendalConfig,
}

impl Config for S3Config {
  type Builder = S3Builder;

  fn into_builder(self) -> Self::Builder {
    S3Builder { config: self }
  }
}

#[derive(Default)]
pub struct S3Builder {
  config: S3Config,
}

impl S3Builder {
  pub fn access_key_id(mut self, access_key_id: impl Into<String>) -> Self {
    self.config.access_key_id = Some(access_key_id.into());
    self
  }

  pub fn secret_access_key(mut self, secret_access_key: impl Into<String>) -> Self {
    self.config.secret_access_key = Some(secret_access_key.into());
    self
  }

  pub fn session_token(mut self, session_token: impl Into<String>) -> Self {
    self.config.session_token = Some(session_token.into());
    self
  }

  pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
    self.config.bucket = bucket.into();
    self
  }

  pub fn region(mut self, region: impl Into<String>) -> Self {
    self.config.region = Some(region.into());
    self
  }

  pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
    self.config.endpoint = Some(endpoint.into());
    self
  }

  pub fn role_arn(mut self, arn: impl Into<String>) -> Self {
    self.config.role_arn = Some(arn.into());
    self
  }

  pub fn role_session_name(mut self, session_name: impl Into<String>) -> Self {
    self.config.role_session_name = Some(session_name.into());
    self
  }

  pub fn external_id(mut self, external_id: impl Into<String>) -> Self {
    self.config.external_id = Some(external_id.into());
    self
  }

  fn build_service(&self) -> opendal::services::S3 {
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
    s
  }
}

impl_opendal_config_for_builder!(S3Builder);

impl Builder for S3Builder {
  type Config = S3Config;

  fn build(self) -> crate::Result<impl Remote> {
    let service = self.build_service();
    let remote = crate::common::opendal::OpendalRemote::new(service, self.config.opendal)?;
    Ok(S3 { remote })
  }
}

pub struct S3 {
  remote: crate::common::opendal::OpendalRemote,
}

impl S3 {
  pub fn builder() -> S3Builder {
    S3Builder::default()
  }
}

#[async_trait]
impl Remote for S3 {
  async fn upload(&self, bundle_name: &str, version: &str, bundle: &Bundle) -> crate::Result<()> {
    self.remote.upload(bundle_name, version, bundle).await
  }

  async fn download(&self, bundle_name: &str, version: &str) -> crate::Result<Bundle> {
    self.remote.download(bundle_name, version).await
  }
}

#[cfg(test)]
mod tests {
  #[skipif::skip_if(missing_env(MINIO_TESTING_ENDPOINT))]
  #[tokio::test]
  async fn smoke() {
    use super::*;
    use std::path::PathBuf;
    use uuid::Uuid;
    use webview_bundle::{AsyncBundleReader, AsyncReader};

    let version = Uuid::new_v4().to_string();
    let s3 = S3::builder()
      .bucket("webview-bundle")
      .endpoint(std::env::var("MINIO_TESTING_ENDPOINT").unwrap())
      .access_key_id("minio_testing")
      .secret_access_key("minio_testing")
      .build()
      .unwrap();
    let mut file = tokio::fs::File::open(
      PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("nextjs.wvb"),
    )
    .await
    .unwrap();
    let bundle = AsyncReader::<Bundle>::read(&mut AsyncBundleReader::new(&mut file))
      .await
      .unwrap();
    s3.upload("nextjs", &version, &bundle).await.unwrap();
    s3.download("nextjs", &version).await.unwrap();
  }
}
