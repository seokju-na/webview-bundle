use crate::bundle::JsBundle;
use crate::remote::JsHttpOptions;
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::remote::uploader::{S3Uploader, S3UploaderBuilder, Uploader};

#[derive(Default)]
#[napi(object, js_name = "S3UploaderOptions")]
pub struct JsS3UploaderOptions {
  pub access_key_id: Option<String>,
  pub secret_access_key: Option<String>,
  pub session_token: Option<String>,
  pub region: Option<String>,
  pub endpoint: Option<String>,
  pub role_arn: Option<String>,
  pub role_session_name: Option<String>,
  pub external_id: Option<String>,

  // config for opendal
  pub write_concurrent: Option<u32>,
  pub write_chunk: Option<u32>,
  pub cache_control: Option<String>,
  pub http: Option<JsHttpOptions>,
}

impl TryFrom<JsS3UploaderOptions> for S3UploaderBuilder {
  type Error = crate::Error;
  fn try_from(value: JsS3UploaderOptions) -> Result<Self, Self::Error> {
    let mut builder = S3Uploader::builder();
    if let Some(access_key_id) = value.access_key_id {
      builder = builder.access_key_id(access_key_id);
    }
    if let Some(secret_access_key) = value.secret_access_key {
      builder = builder.secret_access_key(secret_access_key);
    }
    if let Some(session_token) = value.session_token {
      builder = builder.session_token(session_token);
    }
    if let Some(region) = value.region {
      builder = builder.region(region);
    }
    if let Some(endpoint) = value.endpoint {
      builder = builder.endpoint(endpoint);
    }
    if let Some(role_arn) = value.role_arn {
      builder = builder.role_arn(role_arn);
    }
    if let Some(role_session_name) = value.role_session_name {
      builder = builder.role_session_name(role_session_name);
    }
    if let Some(external_id) = value.external_id {
      builder = builder.external_id(external_id);
    }
    if let Some(write_concurrent) = value.write_concurrent {
      builder = builder.write_concurrent(write_concurrent as usize);
    }
    if let Some(write_chunk) = value.write_chunk {
      builder = builder.write_chunk(write_chunk as usize);
    }
    if let Some(cache_control) = value.cache_control {
      builder = builder.cache_control(cache_control);
    }
    if let Some(http) = value.http {
      builder = builder.http(http.try_into()?);
    }
    Ok(builder)
  }
}

#[napi(js_name = "S3Uploader")]
pub struct JsS3Uploader {
  pub(crate) inner: Arc<S3Uploader>,
}

#[napi]
impl JsS3Uploader {
  #[napi(constructor)]
  pub fn new(bucket: String, options: Option<JsS3UploaderOptions>) -> crate::Result<JsS3Uploader> {
    let builder: S3UploaderBuilder =
      S3UploaderBuilder::try_from(options.unwrap_or_default())?.bucket(bucket);
    let inner = builder.build()?;
    Ok(JsS3Uploader {
      inner: Arc::new(inner),
    })
  }

  #[napi]
  pub async fn upload_bundle(
    &self,
    bundle_name: String,
    version: String,
    bundle: &JsBundle,
  ) -> crate::Result<()> {
    self
      .inner
      .upload_bundle(&bundle_name, &version, &bundle.inner)
      .await?;
    Ok(())
  }
}
