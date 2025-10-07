use crate::bundle::JsBundle;
use crate::remote::JsHttpOptions;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeCallContext;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Status;
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::remote::HttpConfig;
use webview_bundle::remote::{Remote, RemoteBundleInfo};

#[napi(object, js_name = "RemoteOptions")]
pub struct JsRemoteOptions {
  pub http: Option<JsHttpOptions>,
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
  pub integrity: Option<String>,
}

impl From<RemoteBundleInfo> for JsRemoteBundleInfo {
  fn from(value: RemoteBundleInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      integrity: value.integrity,
    }
  }
}

impl From<JsRemoteBundleInfo> for RemoteBundleInfo {
  fn from(value: JsRemoteBundleInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      integrity: value.integrity,
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
  pub fn new(
    endpoint: String,
    options: Option<JsRemoteOptions>,
    #[napi(ts_arg_type = "(data: RemoteOnDownloadData) => void")] on_download: Option<
      Function<JsRemoteOnDownloadData, ()>,
    >,
  ) -> crate::Result<JsRemote> {
    let mut builder = Remote::builder().endpoint(endpoint);
    if let Some(on_download) = on_download {
      let cb = on_download.build_threadsafe_function().build_callback(
        |ctx: ThreadsafeCallContext<(u64, u64)>| {
          Ok(JsRemoteOnDownloadData {
            downloaded_bytes: ctx.value.0 as u32,
            total_bytes: ctx.value.1 as u32,
          })
        },
      )?;
      builder = builder.on_download(move |downloaded_bytes, total_bytes| {
        cb.call(
          (downloaded_bytes, total_bytes),
          ThreadsafeFunctionCallMode::NonBlocking,
        );
      });
    }
    if let Some(options) = options {
      if let Some(http) = options.http {
        builder = builder.http(
          HttpConfig::try_from(http).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
        );
      }
    }
    let inner = builder.build()?;
    Ok(JsRemote {
      inner: Arc::new(inner),
    })
  }

  #[napi]
  pub async fn get_info_all(&self) -> crate::Result<Vec<JsRemoteBundleInfo>> {
    let res = self.inner.get_info_all().await?;
    Ok(res.into_iter().map(JsRemoteBundleInfo::from).collect())
  }

  #[napi]
  pub async fn get_info(&self, bundle_name: String) -> crate::Result<JsRemoteBundleInfo> {
    let res = self.inner.get_info(&bundle_name).await?;
    Ok(JsRemoteBundleInfo::from(res))
  }

  #[napi]
  pub async fn download(&self, info: JsRemoteBundleInfo) -> crate::Result<JsBundle> {
    let inner = self.inner.download(&info.into()).await?;
    Ok(JsBundle { inner })
  }
}
