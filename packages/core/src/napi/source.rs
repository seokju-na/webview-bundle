use crate::napi::bundle::{JsBundle, JsBundleManifest, JsBundleManifestInner};
use crate::source::{BundleSource, BundleSourceVersion};
use napi_derive::napi;
use std::path::Path;
use std::sync::Arc;

#[napi(string_enum = "lowercase", js_name = "BundleSourceVersionType")]
pub enum JsBundleSourceVersionKind {
  Builtin,
  Remote,
}

#[napi(object, js_name = "BundleSourceVersion")]
pub struct JsBundleSourceVersion {
  #[napi(js_name = "type")]
  pub kind: JsBundleSourceVersionKind,
  pub version: String,
}

impl From<BundleSourceVersion> for JsBundleSourceVersion {
  fn from(value: BundleSourceVersion) -> Self {
    match value {
      BundleSourceVersion::Builtin(x) => Self {
        kind: JsBundleSourceVersionKind::Builtin,
        version: x,
      },
      BundleSourceVersion::Remote(x) => Self {
        kind: JsBundleSourceVersionKind::Remote,
        version: x,
      },
    }
  }
}

impl From<JsBundleSourceVersion> for BundleSourceVersion {
  fn from(value: JsBundleSourceVersion) -> Self {
    match value.kind {
      JsBundleSourceVersionKind::Builtin => Self::Builtin(value.version),
      JsBundleSourceVersionKind::Remote => Self::Remote(value.version),
    }
  }
}

#[napi(js_name = "BundleSource")]
pub struct JsBundleSource {
  pub(crate) inner: Arc<BundleSource>,
}

#[napi]
impl JsBundleSource {
  #[napi(constructor)]
  pub fn new(builtin_dir: String, remote_dir: String) -> JsBundleSource {
    let inner = Arc::new(BundleSource::new(
      Path::new(&builtin_dir),
      Path::new(&remote_dir),
    ));
    JsBundleSource { inner }
  }

  #[napi]
  pub async fn get_filepath(&self, bundle_name: String) -> crate::Result<Option<String>> {
    let filepath = self
      .inner
      .get_filepath(&bundle_name)
      .await?
      .map(|x| x.to_string_lossy().to_string());
    Ok(filepath)
  }

  #[napi]
  pub async fn get_version(
    &self,
    bundle_name: String,
  ) -> crate::Result<Option<JsBundleSourceVersion>> {
    let version = self
      .inner
      .get_version(&bundle_name)
      .await?
      .map(JsBundleSourceVersion::from);
    Ok(version)
  }

  #[napi]
  pub async fn set_version(&self, bundle_name: String, version: String) -> crate::Result<()> {
    self.inner.set_version(&bundle_name, &version).await?;
    Ok(())
  }

  #[napi]
  pub async fn is_exists(
    &self,
    bundle_name: String,
    version: JsBundleSourceVersion,
  ) -> crate::Result<bool> {
    let is_exists = self.inner.is_exists(&bundle_name, &version.into()).await?;
    Ok(is_exists)
  }

  #[napi]
  pub async fn fetch(&self, bundle_name: String) -> crate::Result<JsBundle> {
    let inner = self.inner.fetch(&bundle_name).await?;
    Ok(JsBundle { inner })
  }

  #[napi]
  pub async fn fetch_manifest(&self, bundle_name: String) -> crate::Result<JsBundleManifest> {
    let inner = self.inner.fetch_manifest(&bundle_name).await?;
    Ok(JsBundleManifest {
      inner: JsBundleManifestInner::Owned(inner),
    })
  }
}
