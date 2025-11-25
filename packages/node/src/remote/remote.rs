use crate::bundle::JsBundle;
use crate::js::{JsCallback, JsCallbackExt};
use crate::remote::JsHttpOptions;
use napi::bindgen_prelude::*;
use napi::Status;
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::remote::HttpConfig;
use webview_bundle::remote::{Remote, RemoteBundleInfo};

#[napi(object, js_name = "RemoteOptions", object_to_js = false)]
pub struct JsRemoteOptions {
  pub http: Option<JsHttpOptions>,
  #[napi(ts_type = "(data: RemoteOnDownloadData) => void")]
  pub on_download: Option<JsCallback<JsRemoteOnDownloadData, ()>>,
}

#[napi(object, js_name = "RemoteOnDownloadData")]
pub struct JsRemoteOnDownloadData {
  pub downloaded_bytes: u32,
  pub total_bytes: u32,
}

#[napi(object, js_name = "RemoteBundleInfo")]
pub struct JsRemoteBundleInfo {
  pub name: String,
  pub version: String,
  pub etag: Option<String>,
  pub integrity: Option<String>,
  pub signature: Option<String>,
}

impl From<RemoteBundleInfo> for JsRemoteBundleInfo {
  fn from(value: RemoteBundleInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
    }
  }
}

impl From<JsRemoteBundleInfo> for RemoteBundleInfo {
  fn from(value: JsRemoteBundleInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
    }
  }
}

#[napi(js_name = "Remote")]
pub struct JsRemote {
  pub(crate) inner: Arc<Remote>,
}

#[napi]
impl JsRemote {
  #[napi(constructor)]
  pub fn new(endpoint: String, options: Option<JsRemoteOptions>) -> crate::Result<JsRemote> {
    let mut builder = Remote::builder().endpoint(endpoint);
    if let Some(options) = options {
      if let Some(http) = options.http {
        builder = builder.http(
          HttpConfig::try_from(http).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
        );
      }
      if let Some(on_download) = options.on_download {
        builder = builder.on_download(move |downloaded_bytes, total_bytes| {
          let on_download_fn = Arc::clone(&on_download);
          let _ = on_download_fn.invoke_sync(JsRemoteOnDownloadData {
            downloaded_bytes: downloaded_bytes as u32,
            total_bytes: total_bytes as u32,
          });
        });
      }
    }
    let inner = builder.build()?;
    Ok(JsRemote {
      inner: Arc::new(inner),
    })
  }

  #[napi]
  pub async fn list_bundles(&self) -> crate::Result<Vec<String>> {
    let bundles = self.inner.list_bundles().await?;
    Ok(bundles)
  }

  #[napi]
  pub async fn get_info(&self, bundle_name: String) -> crate::Result<JsRemoteBundleInfo> {
    let info = self.inner.get_current_info(&bundle_name).await?;
    Ok(info.into())
  }

  #[napi]
  pub async fn download(
    &self,
    bundle_name: String,
  ) -> crate::Result<(JsRemoteBundleInfo, JsBundle, Buffer)> {
    let (info, inner, data) = self.inner.download(&bundle_name).await?;
    Ok((info.into(), JsBundle { inner }, data.into()))
  }

  #[napi]
  pub async fn download_version(
    &self,
    bundle_name: String,
    version: String,
  ) -> crate::Result<(JsRemoteBundleInfo, JsBundle, Buffer)> {
    let (info, inner, data) = self.inner.download_version(&bundle_name, &version).await?;
    Ok((info.into(), JsBundle { inner }, data.into()))
  }
}
