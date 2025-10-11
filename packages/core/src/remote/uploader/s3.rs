use crate::remote::uploader::Uploader;
use crate::remote::HttpConfig;
use crate::{impl_opendal_config_for_builder, BundleWriter, Writer, MIME_TYPE};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Default, Clone)]
#[non_exhaustive]
pub struct S3UploaderConfig {
  pub access_key_id: Option<String>,
  pub secret_access_key: Option<String>,
  pub session_token: Option<String>,
  pub bucket: String,
  pub region: Option<String>,
  pub endpoint: Option<String>,
  pub role_arn: Option<String>,
  pub role_session_name: Option<String>,
  pub external_id: Option<String>,
  pub(crate) opendal: crate::remote::opendal::OpendalConfig,
}

#[derive(Default)]
pub struct S3UploaderBuilder {
  config: S3UploaderConfig,
}

impl S3UploaderBuilder {
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

  #[must_use]
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

  pub fn build(self) -> crate::Result<S3Uploader> {
    let service = self.build_service();
    let mut op = opendal::Operator::new(service)
      .map_err(|e| {
        if e.kind() == opendal::ErrorKind::ConfigInvalid {
          return crate::Error::invalid_remote_config(e.to_string());
        }
        crate::Error::Opendal(e)
      })?
      .finish();
    if let Some(ref http_config) = self.config.opendal.http {
      let mut http = reqwest::ClientBuilder::new();
      http = http_config.apply(http);
      let client = http.build()?;
      op = op.layer(opendal::layers::HttpClientLayer::new(
        opendal::raw::HttpClient::with(client),
      ));
    }
    Ok(S3Uploader {
      config: self.config,
      op,
    })
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

impl_opendal_config_for_builder!(S3UploaderBuilder);

pub struct S3Uploader {
  config: S3UploaderConfig,
  op: opendal::Operator,
}

impl S3Uploader {
  pub fn builder() -> S3UploaderBuilder {
    S3UploaderBuilder::default()
  }
}

#[async_trait]
impl Uploader for S3Uploader {
  // TODO: integrity
  async fn upload_bundle(
    &self,
    bundle_name: &str,
    version: &str,
    bundle: &crate::Bundle,
  ) -> crate::Result<()> {
    let path = self.config.opendal.resolve_path(bundle_name, version);
    let mut writer = self
      .op
      .writer_with(&path)
      .content_type(MIME_TYPE)
      .user_metadata([
        ("webview-bundle-name".to_string(), bundle_name.to_string()),
        ("webview-bundle-version".to_string(), version.to_string()),
      ]);
    if let Some(concurrent) = self.config.opendal.write_concurrent {
      writer = writer.concurrent(concurrent);
    }
    if let Some(chunk) = self.config.opendal.write_chunk {
      writer = writer.chunk(chunk);
    }
    if let Some(ref content_disposition) = self
      .config
      .opendal
      .resolve_content_disposition(bundle_name, version)
    {
      writer = writer.content_disposition(content_disposition);
    }
    if let Some(ref cache_control) = self.config.opendal.cache_control {
      writer = writer.cache_control(cache_control);
    }
    let mut w = writer.await?;
    let mut data = vec![];
    BundleWriter::new(&mut data).write(bundle)?;
    w.write(data).await?;
    w.close().await?;
    Ok(())
  }
}
