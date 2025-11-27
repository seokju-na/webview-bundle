#[cfg(feature = "remote")]
use crate::remote::RemoteBundleInfo;
use crate::source::BundleMetadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use tokio::sync::OnceCell;

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
  versions: RwLock<OnceCell<HashMap<String, String>>>,
  metadata: RwLock<HashMap<String, OnceCell<BundleMetadata>>>,
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
      versions: Default::default(),
      metadata: Default::default(),
      versions_filepath_fn: Box::new(default_versions_filepath),
      metadata_filepath_fn: Box::new(default_metadata_filepath),
    }
  }

  pub fn get_versions_filepath(&self) -> PathBuf {
    (self.versions_filepath_fn)(&self.base_dir)
  }

  pub fn get_metadata_filepath(&self, bundle_name: &str, version: &str) -> PathBuf {
    (self.metadata_filepath_fn)(&self.base_dir, bundle_name, version)
  }

  pub async fn load_version(&self, bundle_name: &str) -> crate::Result<Option<String>> {
    let v = {
      let mut version = None;
      let versions = self.versions.read().unwrap();
      if let Some(vmap) = versions.get() {
        if let Some(v) = vmap.get(bundle_name) {
          version = Some(v.to_string());
        }
      }
      version
    };
    if v.is_some() {
      return Ok(v);
    }
    let versions_filepath = self.get_versions_filepath();
    let versions = self.versions.write().unwrap();
    let vmap = versions
      .get_or_try_init(|| async {
        let raw = tokio::fs::read(&versions_filepath).await?;
        let versions: HashMap<String, String> = serde_json::from_slice(&raw)?;
        Ok::<HashMap<String, String>, crate::Error>(versions)
      })
      .await?;
    todo!()
  }

  async fn save_inner(&self) -> crate::Result<()> {
    let raw = {
      let json = self.data.read().unwrap();
      serde_json::to_vec(&*json)
    }?;
    tokio::fs::write(&self.filepath, raw).await?;
    Ok(())
  }

  fn update_inner(&self, update_data: BundleManifestData) {
    let mut data = self.data.write().unwrap();
    *data = update_data;
  }
}

impl BundleManifest<ReadWrite> {
  pub async fn save(&self) -> crate::Result<()> {
    self.save_inner().await
  }

  pub fn update(&self, update_data: BundleManifestData) {
    self.update_inner(update_data);
  }
}
