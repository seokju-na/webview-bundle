use crate::bundle::Bundle;
use crate::js::{JsCallback, JsCallbackExt};
use crate::remote::HttpOptions;
use napi::bindgen_prelude::*;
use napi::Status;
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::remote;
use webview_bundle::remote::HttpConfig;

#[napi(object, object_to_js = false)]
pub struct RemoteOptions {
  pub http: Option<HttpOptions>,
  #[napi(ts_type = "(data: RemoteOnDownloadData) => void")]
  pub on_download: Option<JsCallback<RemoteOnDownloadData, ()>>,
}

#[napi(object)]
pub struct RemoteOnDownloadData {
  pub downloaded_bytes: u32,
  pub total_bytes: u32,
}

#[napi(object)]
pub struct RemoteBundleInfo {
  pub name: String,
  pub version: String,
  pub etag: Option<String>,
  pub integrity: Option<String>,
  pub signature: Option<String>,
  pub last_modified: Option<String>,
}

impl From<remote::RemoteBundleInfo> for RemoteBundleInfo {
  fn from(value: remote::RemoteBundleInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
      last_modified: value.last_modified,
    }
  }
}

impl From<RemoteBundleInfo> for remote::RemoteBundleInfo {
  fn from(value: RemoteBundleInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
      last_modified: value.last_modified,
    }
  }
}

#[napi]
pub struct Remote {
  pub(crate) inner: Arc<remote::Remote>,
}

#[napi]
impl Remote {
  #[napi(constructor)]
  pub fn new(endpoint: String, options: Option<RemoteOptions>) -> crate::Result<Remote> {
    let mut builder = remote::Remote::builder().endpoint(endpoint);
    if let Some(options) = options {
      if let Some(http) = options.http {
        builder = builder.http(
          HttpConfig::try_from(http).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
        );
      }
      if let Some(on_download) = options.on_download {
        builder = builder.on_download(move |downloaded_bytes, total_bytes| {
          let on_download_fn = Arc::clone(&on_download);
          let _ = on_download_fn.invoke_sync(RemoteOnDownloadData {
            downloaded_bytes: downloaded_bytes as u32,
            total_bytes: total_bytes as u32,
          });
        });
      }
    }
    let inner = builder.build()?;
    Ok(Remote {
      inner: Arc::new(inner),
    })
  }

  #[napi]
  pub async fn list_bundles(&self) -> crate::Result<Vec<String>> {
    let bundles = self.inner.list_bundles().await?;
    Ok(bundles)
  }

  #[napi]
  pub async fn get_info(&self, bundle_name: String) -> crate::Result<RemoteBundleInfo> {
    let info = self.inner.get_current_info(&bundle_name).await?;
    Ok(info.into())
  }

  #[napi]
  pub async fn download(
    &self,
    bundle_name: String,
  ) -> crate::Result<(RemoteBundleInfo, Bundle, Buffer)> {
    let (info, inner, data) = self.inner.download(&bundle_name).await?;
    Ok((info.into(), Bundle { inner }, data.into()))
  }

  #[napi]
  pub async fn download_version(
    &self,
    bundle_name: String,
    version: String,
  ) -> crate::Result<(RemoteBundleInfo, Bundle, Buffer)> {
    let (info, inner, data) = self.inner.download_version(&bundle_name, &version).await?;
    Ok((info.into(), Bundle { inner }, data.into()))
  }
}
