use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BundleCurrentVersionsData {
  pub versions: HashMap<String, String>,
}

pub struct BundleCurrent {
  filepath: PathBuf,
  data: OnceCell<Arc<RwLock<BundleCurrentVersionsData>>>,
  sync: OnceCell<u64>,
}

impl BundleCurrent {
  pub fn new(filepath: &Path) -> Self {
    Self {
      filepath: filepath.to_path_buf(),
      data: OnceCell::new(),
      sync: OnceCell::new(),
    }
  }

  pub async fn list_versions(&self) -> crate::Result<HashMap<String, String>> {
    let data = self.load().await?.read().await;
    Ok(data.versions.clone())
  }

  pub async fn load_version(&self, bundle_name: &str) -> crate::Result<Option<String>> {
    let data = self.load().await?.read().await;
    let version = data.versions.get(bundle_name).cloned();
    Ok(version)
  }

  pub async fn update_version(&self, bundle_name: &str, version: &str) -> crate::Result<()> {
    let mut data = self.load().await?.write().await;
    data
      .versions
      .insert(bundle_name.to_string(), version.to_string());
    Ok(())
  }

  pub async fn remove_version(&self, bundle_name: &str) -> crate::Result<bool> {
    let mut data = self.load().await?.write().await;
    let removed = data.versions.remove(bundle_name).is_some();
    Ok(removed)
  }

  pub async fn save(&self) -> crate::Result<()> {
    let raw = {
      let data = self.load().await?.read().await;
      serde_json::to_vec(&*data)
    }?;
    if let Some(dir) = self.filepath.parent() {
      tokio::fs::create_dir_all(dir).await?;
    }
    tokio::fs::write(&self.filepath, raw).await?;
    Ok(())
  }

  async fn load(&self) -> crate::Result<&RwLock<BundleCurrentVersionsData>> {
    let data = self
      .data
      .get_or_try_init(|| async {
        if !tokio::fs::try_exists(&self.filepath).await? {
          return Ok::<Arc<RwLock<BundleCurrentVersionsData>>, crate::Error>(Default::default());
        }
        let raw = tokio::fs::read(&self.filepath).await?;
        let data: BundleCurrentVersionsData = serde_json::from_slice(&raw)?;
        Ok::<Arc<RwLock<BundleCurrentVersionsData>>, crate::Error>(Arc::new(RwLock::new(data)))
      })
      .await?;

    // todo

    Ok(data)
  }
}
