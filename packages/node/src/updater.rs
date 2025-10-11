use crate::remote::{JsRemote, JsRemoteBundleInfo};
use crate::source::JsBundleSource;
use napi_derive::napi;
use webview_bundle::updater::{BundleUpdateInfo, Updater};

#[napi(object, js_name = "BundleUpdateInfo")]
pub struct JsBundleUpdateInfo {
  pub name: String,
  pub version: String,
  pub local_version: Option<String>,
  pub is_available: bool,
  pub integrity: Option<String>,
}

impl From<BundleUpdateInfo> for JsBundleUpdateInfo {
  fn from(value: BundleUpdateInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      local_version: value.local_version,
      is_available: value.is_available,
      integrity: value.integrity,
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
      integrity: value.integrity,
    }
  }
}

#[napi(js_name = "Updater")]
pub struct JsUpdater {
  pub(crate) inner: Updater,
}

#[napi]
impl JsUpdater {
  #[napi(constructor)]
  pub fn new(source: &JsBundleSource, remote: &JsRemote) -> JsUpdater {
    let source = source.inner.clone();
    let remote = remote.inner.clone();
    JsUpdater {
      inner: Updater::new(source, remote),
    }
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
