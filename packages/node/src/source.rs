use crate::bundle::JsBundle;
use crate::bundle::JsBundleDescriptor;
use crate::bundle::JsBundleDescriptorInner;
use napi_derive::napi;
use std::path::Path;
use std::sync::Arc;
use webview_bundle::source::{
  BundleManifestMetadata, BundleSource, BundleSourceKind, BundleSourceVersion, ListBundleItem,
};

#[napi(string_enum = "lowercase", js_name = "BundleSourceType")]
pub enum JsBundleSourceKind {
  Builtin,
  Remote,
}

impl From<BundleSourceKind> for JsBundleSourceKind {
  fn from(value: BundleSourceKind) -> Self {
    match value {
      BundleSourceKind::Builtin => Self::Builtin,
      BundleSourceKind::Remote => Self::Remote,
    }
  }
}

#[napi(object, js_name = "BundleSourceVersion")]
pub struct JsBundleSourceVersion {
  #[napi(js_name = "type")]
  pub kind: JsBundleSourceKind,
  pub version: String,
}

impl From<BundleSourceVersion> for JsBundleSourceVersion {
  fn from(value: BundleSourceVersion) -> Self {
    Self {
      kind: value.kind.into(),
      version: value.version,
    }
  }
}

#[napi(object, js_name = "BundleManifestMetadata")]
pub struct JsBundleManifestMetadata {
  pub etag: Option<String>,
  pub integrity: Option<String>,
  pub signature: Option<String>,
  pub last_modified: Option<String>,
}

impl From<BundleManifestMetadata> for JsBundleManifestMetadata {
  fn from(value: BundleManifestMetadata) -> Self {
    Self {
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
      last_modified: value.last_modified,
    }
  }
}

#[napi(object, js_name = "ListBundleItem")]
pub struct JsListBundleItem {
  #[napi(js_name = "type")]
  pub kind: JsBundleSourceKind,
  pub name: String,
  pub version: String,
  pub current: bool,
  pub metadata: BundleManifestMetadata,
}

impl From<ListBundleItem> for JsListBundleItem {
  fn from(value: ListBundleItem) -> Self {
    Self {
      kind: value.kind.into(),
      name: value.item.name,
      version: value.item.version,
      current: value.item.current,
      metadata: value.item.metadata.into(),
    }
  }
}

#[napi(object, js_name = "BundleSourceConfig")]
pub struct JsBundleSourceConfig {
  pub builtin_dir: String,
  pub remote_dir: String,
  pub builtin_manifest_filepath: Option<String>,
  pub remote_manifest_filepath: Option<String>,
}

#[napi(js_name = "BundleSource")]
pub struct JsBundleSource {
  pub(crate) inner: Arc<BundleSource>,
}

#[napi]
impl JsBundleSource {
  #[napi(constructor)]
  pub fn new(config: JsBundleSourceConfig) -> JsBundleSource {
    let mut builder = BundleSource::builder()
      .builtin_dir(config.builtin_dir)
      .remote_dir(config.remote_dir);
    if let Some(builtin_manifest) = config.builtin_manifest_filepath {
      builder = builder.builtin_manifest_filepath(builtin_manifest);
    }
    if let Some(remote_manifest) = config.remote_manifest_filepath {
      builder = builder.remote_manifest_filepath(remote_manifest);
    }
    let source = builder.build();
    JsBundleSource {
      inner: Arc::new(source),
    }
  }

  #[napi]
  pub async fn list_bundles(&self) -> crate::Result<Vec<JsListBundleItem>> {
    let items = self
      .inner
      .list_bundles()
      .await?
      .into_iter()
      .map(JsListBundleItem::from)
      .collect::<Vec<_>>();
    Ok(items)
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
  pub async fn save_versions(&self) -> crate::Result<()> {
    self.inner.save_versions().await?;
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
  pub async fn fetch_manifest(&self, bundle_name: String) -> crate::Result<JsBundleDescriptor> {
    let inner = self.inner.fetch_descriptor(&bundle_name).await?;
    Ok(JsBundleDescriptor {
      inner: JsBundleDescriptorInner::Owned(inner),
    })
  }
}
