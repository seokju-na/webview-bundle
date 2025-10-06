use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct BundleVersionsJson {
  versions: HashMap<String, String>,
}

pub trait BundleVersionsMode: Send + Sync + 'static {}
pub struct ReadOnly;
impl BundleVersionsMode for ReadOnly {}
pub struct ReadWrite;
impl BundleVersionsMode for ReadWrite {}

pub struct BundleVersions<Mode: BundleVersionsMode> {
  _mode: std::marker::PhantomData<Mode>,
  filepath: PathBuf,
  json: RwLock<BundleVersionsJson>,
}

impl<Mode> BundleVersions<Mode>
where
  Mode: BundleVersionsMode,
{
  pub fn new(filepath: &Path, _mode: Mode) -> Self {
    Self {
      _mode: std::marker::PhantomData,
      filepath: filepath.to_path_buf(),
      json: RwLock::new(Default::default()),
    }
  }

  pub async fn load(filepath: &Path, _mode: Mode) -> crate::Result<Self> {
    let filepath = filepath.to_path_buf();
    let raw = tokio::fs::read(&filepath).await?;
    let json: BundleVersionsJson = serde_json::from_slice(&raw)?;
    Ok(Self {
      _mode: std::marker::PhantomData,
      filepath,
      json: RwLock::new(json),
    })
  }

  pub fn filepath(&self) -> &Path {
    &self.filepath
  }

  async fn save_inner(&self) -> crate::Result<()> {
    let raw = {
      let json = self.json.read().unwrap();
      serde_json::to_vec(&*json)
    }?;
    tokio::fs::write(&self.filepath, raw).await?;
    Ok(())
  }

  pub fn get_version(&self, bundle_name: &str) -> Option<String> {
    let json = self.json.read().unwrap();
    let version = json.versions.get(bundle_name).cloned();
    version
  }

  fn set_version_inner(&self, bundle_name: &str, version: &str) {
    let mut json = self.json.write().unwrap();
    json
      .versions
      .insert(bundle_name.to_string(), version.to_string());
  }
}

impl BundleVersions<ReadWrite> {
  pub async fn save(&self) -> crate::Result<()> {
    self.save_inner().await
  }

  pub fn set_version(&self, bundle_name: &str, version: &str) {
    self.set_version_inner(bundle_name, version);
  }
}
