use crate::source::BundleMetadata;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

pub trait BundleManifestMode: Send + Sync + 'static {}
pub struct ReadOnly;
impl BundleManifestMode for ReadOnly {}
pub struct ReadWrite;
impl BundleManifestMode for ReadWrite {}

pub type VersionsFilepathFn = fn(base_dir: &Path) -> PathBuf;
pub type MetadataFilepathFn = fn(base_dir: &Path, bundle_name: &str, version: &str) -> PathBuf;

fn default_versions_filepath(base_dir: &Path) -> PathBuf {
  base_dir.join("versions.json")
}

fn default_metadata_filepath(base_dir: &Path, bundle_name: &str, version: &str) -> PathBuf {
  base_dir
    .join(bundle_name)
    .join(version)
    .join("metadata.json")
}

pub struct BundleManifest<Mode: BundleManifestMode> {
  _mode: std::marker::PhantomData<Mode>,
  base_dir: PathBuf,
  list_bundles_cache: OnceCell<Vec<String>>,
  versions: OnceCell<RwLock<HashMap<String, String>>>,
  metadata: RwLock<HashMap<(String, String), OnceCell<Arc<BundleMetadata>>>>,
  versions_filepath_fn: Box<VersionsFilepathFn>,
  metadata_filepath_fn: Box<MetadataFilepathFn>,
}

impl<Mode> BundleManifest<Mode>
where
  Mode: BundleManifestMode,
{
  pub fn new(base_dir: &Path, _mode: Mode) -> Self {
    Self {
      _mode: std::marker::PhantomData,
      base_dir: base_dir.to_path_buf(),
      list_bundles_cache: Default::default(),
      versions: Default::default(),
      metadata: Default::default(),
      versions_filepath_fn: Box::new(default_versions_filepath),
      metadata_filepath_fn: Box::new(default_metadata_filepath),
    }
  }

  pub fn new_with(
    base_dir: &Path,
    versions_filepath_fn: Option<Box<VersionsFilepathFn>>,
    metadata_filepath_fn: Option<Box<MetadataFilepathFn>>,
    _mode: Mode,
  ) -> Self {
    Self {
      _mode: std::marker::PhantomData,
      base_dir: base_dir.to_path_buf(),
      list_bundles_cache: Default::default(),
      versions: Default::default(),
      metadata: Default::default(),
      versions_filepath_fn: versions_filepath_fn
        .unwrap_or_else(|| Box::new(default_versions_filepath)),
      metadata_filepath_fn: metadata_filepath_fn
        .unwrap_or_else(|| Box::new(default_metadata_filepath)),
    }
  }

  pub fn base_dir(&self) -> &Path {
    self.base_dir.as_path()
  }

  pub fn versions_filepath(&self) -> PathBuf {
    (self.versions_filepath_fn)(&self.base_dir)
  }

  pub fn metadata_filepath(&self, bundle_name: &str, version: &str) -> PathBuf {
    (self.metadata_filepath_fn)(&self.base_dir, bundle_name, version)
  }

  pub async fn load_version(&self, bundle_name: &str) -> crate::Result<Option<String>> {
    let versions = self.load_versions().await?;
    let read = versions.read().await;
    Ok(read.get(bundle_name).cloned())
  }

  pub async fn load_metadata(
    &self,
    bundle_name: &str,
    version: &str,
  ) -> crate::Result<Arc<BundleMetadata>> {
    let key = (bundle_name.to_string(), version.to_string());
    if let Some(m) = {
      let metadata = self.metadata.read().await;
      metadata.get(&key).and_then(|x| x.get()).cloned()
    } {
      return Ok(m);
    }
    let metadata_filepath = self.metadata_filepath(bundle_name, version);
    let metadata = {
      let mut metadata = self.metadata.write().await;
      metadata
        .entry(key)
        .or_insert_with(OnceCell::new)
        .get_or_try_init(|| async {
          let metadata = BundleMetadata::load(&metadata_filepath).await?;
          Ok::<Arc<BundleMetadata>, crate::Error>(Arc::new(metadata))
        })
        .await?
        .clone()
    };
    Ok(metadata)
  }

  async fn load_versions(&self) -> crate::Result<&RwLock<HashMap<String, String>>> {
    let versions_filepath = self.versions_filepath();
    let versions = self
      .versions
      .get_or_try_init(|| async {
        let raw = tokio::fs::read(&versions_filepath).await?;
        let versions: HashMap<String, String> = serde_json::from_slice(&raw)?;
        Ok::<RwLock<HashMap<String, String>>, crate::Error>(RwLock::new(versions))
      })
      .await?;
    Ok(versions)
  }

  async fn update_version_inner(&self, bundle_name: &str, version: &str) -> crate::Result<()> {
    let versions = self.load_versions().await?;
    let mut write = versions.write().await;
    write.insert(bundle_name.to_string(), version.to_string());
    Ok(())
  }

  async fn save_versions_inner(&self) -> crate::Result<()> {
    let data = {
      let versions = self.load_versions().await?.read().await;
      let raw = serde_json::to_vec(&*versions)?;
      raw
    };
    let versions_filepath = self.versions_filepath();
    tokio::fs::write(&versions_filepath, &data).await?;
    Ok(())
  }
}

impl BundleManifest<ReadWrite> {
  pub async fn update_version(&self, bundle_name: &str, version: &str) -> crate::Result<()> {
    self.update_version_inner(bundle_name, version).await
  }

  pub async fn save_versions(&self) -> crate::Result<()> {
    self.save_versions_inner().await
  }
}

#[cfg(test)]
mod tests {}

// list bundles => source에다가 두기
// 캐시 해둔거
