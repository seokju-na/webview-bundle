use crate::bundle::Bundle;
use crate::bundle::BundleDescriptor;
use crate::bundle::BundleDescriptorInner;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;
use wvb::source;

#[napi(string_enum = "lowercase")]
pub enum BundleSourceKind {
  Builtin,
  Remote,
}

impl From<source::BundleSourceKind> for BundleSourceKind {
  fn from(value: source::BundleSourceKind) -> Self {
    match value {
      source::BundleSourceKind::Builtin => Self::Builtin,
      source::BundleSourceKind::Remote => Self::Remote,
    }
  }
}

#[napi(object)]
pub struct BundleSourceVersion {
  #[napi(js_name = "type")]
  pub kind: BundleSourceKind,
  pub version: String,
}

impl From<source::BundleSourceVersion> for BundleSourceVersion {
  fn from(value: source::BundleSourceVersion) -> Self {
    Self {
      kind: value.kind.into(),
      version: value.version,
    }
  }
}

#[napi(object)]
pub struct BundleManifestMetadata {
  pub etag: Option<String>,
  pub integrity: Option<String>,
  pub signature: Option<String>,
  pub last_modified: Option<String>,
}

impl From<source::BundleManifestMetadata> for BundleManifestMetadata {
  fn from(value: source::BundleManifestMetadata) -> Self {
    Self {
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
      last_modified: value.last_modified,
    }
  }
}

impl From<BundleManifestMetadata> for source::BundleManifestMetadata {
  fn from(value: BundleManifestMetadata) -> Self {
    Self {
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
      last_modified: value.last_modified,
    }
  }
}

#[napi]
pub enum BundleManifestVersion {
  V1 = 1,
}

#[napi(object)]
pub struct BundleManifestEntry {
  pub versions: HashMap<String, BundleManifestMetadata>,
  pub current_version: String,
}

#[napi(object)]
pub struct BundleManifestData {
  #[napi(ts_type = "1")]
  pub manifest_version: BundleManifestVersion,
  pub entries: HashMap<String, BundleManifestEntry>,
}

#[napi(object)]
pub struct ListBundleItem {
  #[napi(js_name = "type")]
  pub kind: BundleSourceKind,
  pub name: String,
  pub version: String,
  pub current: bool,
  pub metadata: BundleManifestMetadata,
}

impl From<source::ListBundleItem> for ListBundleItem {
  fn from(value: source::ListBundleItem) -> Self {
    Self {
      kind: value.kind.into(),
      name: value.item.name,
      version: value.item.version,
      current: value.item.current,
      metadata: value.item.metadata.into(),
    }
  }
}

#[napi(object)]
pub struct BundleSourceConfig {
  pub builtin_dir: String,
  pub remote_dir: String,
  pub builtin_manifest_filepath: Option<String>,
  pub remote_manifest_filepath: Option<String>,
}

#[napi]
pub struct BundleSource {
  pub(crate) inner: Arc<source::BundleSource>,
}

#[napi]
impl BundleSource {
  #[napi(constructor)]
  pub fn new(config: BundleSourceConfig) -> BundleSource {
    let mut builder = source::BundleSource::builder()
      .builtin_dir(config.builtin_dir)
      .remote_dir(config.remote_dir);
    if let Some(builtin_manifest) = config.builtin_manifest_filepath {
      builder = builder.builtin_manifest_filepath(builtin_manifest);
    }
    if let Some(remote_manifest) = config.remote_manifest_filepath {
      builder = builder.remote_manifest_filepath(remote_manifest);
    }
    let source = builder.build();
    BundleSource {
      inner: Arc::new(source),
    }
  }

  #[napi]
  pub async fn list_bundles(&self) -> crate::Result<Vec<ListBundleItem>> {
    let items = self
      .inner
      .list_bundles()
      .await?
      .into_iter()
      .map(ListBundleItem::from)
      .collect::<Vec<_>>();
    Ok(items)
  }

  #[napi]
  pub async fn load_version(
    &self,
    bundle_name: String,
  ) -> crate::Result<Option<BundleSourceVersion>> {
    let version = self.inner.load_version(&bundle_name).await?;
    Ok(version.map(Into::into))
  }

  #[napi]
  pub async fn update_version(&self, bundle_name: String, version: String) -> crate::Result<()> {
    self.inner.update_version(&bundle_name, &version).await?;
    Ok(())
  }

  #[napi]
  pub async fn filepath(&self, bundle_name: String) -> crate::Result<String> {
    let filepath = self.inner.filepath(&bundle_name).await?;
    Ok(filepath.to_string_lossy().to_string())
  }

  #[napi]
  pub async fn fetch(&self, bundle_name: String) -> crate::Result<Bundle> {
    let inner = self.inner.fetch(&bundle_name).await?;
    Ok(Bundle { inner })
  }

  #[napi]
  pub async fn fetch_descriptor(&self, bundle_name: String) -> crate::Result<BundleDescriptor> {
    let inner = self.inner.fetch_descriptor(&bundle_name).await?;
    Ok(BundleDescriptor {
      inner: BundleDescriptorInner::Owned(inner),
    })
  }

  #[napi]
  pub async fn write_remote_bundle(
    &self,
    bundle_name: String,
    version: String,
    bundle: &Bundle,
    metadata: BundleManifestMetadata,
  ) -> crate::Result<()> {
    self
      .inner
      .write_remote_bundle(&bundle_name, &version, &bundle.inner, metadata.into())
      .await?;
    Ok(())
  }
}
