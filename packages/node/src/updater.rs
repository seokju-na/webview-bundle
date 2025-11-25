use crate::integrity::JsIntegrityPolicy;
use crate::js::{JsCallback, JsCallbackExt};
use crate::remote::{JsRemote, JsRemoteBundleInfo};
use crate::signature::JsSignatureVerifier;
use crate::source::JsBundleSource;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::integrity::IntegrityChecker;
use webview_bundle::updater::{BundleUpdateInfo, Updater, UpdaterConfig};

#[napi(object, js_name = "BundleUpdateInfo")]
pub struct JsBundleUpdateInfo {
  pub name: String,
  pub version: String,
  pub local_version: Option<String>,
  pub is_available: bool,
  pub etag: Option<String>,
  pub integrity: Option<String>,
  pub signature: Option<String>,
}

impl From<BundleUpdateInfo> for JsBundleUpdateInfo {
  fn from(value: BundleUpdateInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      local_version: value.local_version,
      is_available: value.is_available,
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
    }
  }
}

impl From<JsBundleUpdateInfo> for BundleUpdateInfo {
  fn from(value: JsBundleUpdateInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      local_version: value.local_version,
      is_available: value.is_available,
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
    }
  }
}

#[napi(object, js_name = "UpdaterOptions", object_to_js = false)]
pub struct JsUpdaterOptions {
  pub integrity_policy: Option<JsIntegrityPolicy>,
  #[napi(ts_type = "(data: Uint8Array, integrity: string) => Promise<boolean>")]
  pub integrity_checker: Option<JsCallback<(Buffer, String), Promise<bool>>>,
  #[napi(
    ts_type = "SignatureVerifierOptions | ((data: Uint8Array, signature: string) => Promise<boolean>)"
  )]
  pub signature_verifier: Option<JsSignatureVerifier>,
}

impl From<JsUpdaterOptions> for UpdaterConfig {
  fn from(value: JsUpdaterOptions) -> Self {
    let mut config = UpdaterConfig::default();
    if let Some(policy) = value.integrity_policy {
      config = config.integrity_policy(policy.into());
    }
    if let Some(checker) = value.integrity_checker {
      config = config.integrity_checker(IntegrityChecker::Custom(Arc::new(
        move |data, signature| {
          let buffer = Buffer::from(data);
          let signature = signature.to_string();
          let callback = Arc::clone(&checker);
          Box::pin(async move {
            let ret = callback.invoke_async((buffer, signature)).await?.await?;
            Ok(ret)
          })
        },
      )));
    }
    if let Some(verifier) = value.signature_verifier {
      config = config.signature_verifier(verifier.inner);
    }
    config
  }
}

#[napi(js_name = "Updater")]
pub struct JsUpdater {
  pub(crate) inner: Updater,
}

#[napi]
impl JsUpdater {
  #[napi(constructor)]
  pub fn new(
    source: &JsBundleSource,
    remote: &JsRemote,
    options: Option<JsUpdaterOptions>,
  ) -> crate::Result<JsUpdater> {
    let source = source.inner.clone();
    let remote = remote.inner.clone();
    Ok(JsUpdater {
      inner: Updater::new(source, remote, options.map(Into::into)),
    })
  }

  #[napi]
  pub async fn list_remotes(&self) -> crate::Result<Vec<String>> {
    let remotes = self.inner.list_remotes().await?;
    Ok(remotes)
  }

  #[napi]
  pub async fn get_update(&self, bundle_name: String) -> crate::Result<JsBundleUpdateInfo> {
    let update = self.inner.get_update(&bundle_name).await?;
    Ok(JsBundleUpdateInfo::from(update))
  }

  #[napi]
  pub async fn download_update(
    &self,
    bundle_name: String,
    version: Option<String>,
  ) -> crate::Result<JsRemoteBundleInfo> {
    let info = self.inner.download_update(bundle_name, version).await?;
    Ok(info.into())
  }

  #[napi]
  pub async fn apply_update(&self, bundle_name: String, version: String) -> crate::Result<()> {
    self.inner.apply_update(bundle_name, version).await?;
    Ok(())
  }
}
