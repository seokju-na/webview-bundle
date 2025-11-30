use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use tokio::sync::{OnceCell, RwLock};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum BundleManifestVersion {
  #[default]
  V1 = 1,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BundleManifestMetadata {
  pub etag: Option<String>,
  pub integrity: Option<String>,
  pub signature: Option<String>,
  pub last_modified: Option<u64>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BundleManifestData {
  pub manifest_version: BundleManifestVersion,
  pub entries: HashMap<String, HashMap<String, BundleManifestMetadata>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListBundleManifestItem {
  pub name: String,
  pub version: String,
  #[serde(flatten)]
  pub metadata: BundleManifestMetadata,
}

pub trait BundleManifestMode: Send + Sync + 'static {}
pub struct ReadOnly;
impl BundleManifestMode for ReadOnly {}
pub struct ReadWrite;
impl BundleManifestMode for ReadWrite {}

pub struct BundleManifest<Mode: BundleManifestMode> {
  _mode: std::marker::PhantomData<Mode>,
  filepath: PathBuf,
  data: OnceCell<RwLock<BundleManifestData>>,
}

impl<Mode> BundleManifest<Mode>
where
  Mode: BundleManifestMode,
{
  pub fn new(filepath: &Path, _mode: Mode) -> Self {
    Self {
      _mode: std::marker::PhantomData,
      filepath: filepath.to_path_buf(),
      data: Default::default(),
    }
  }

  pub async fn list_entries(&self) -> crate::Result<Vec<ListBundleManifestItem>> {
    let data = self.load().await?.read().await;
    let mut items = vec![];
    for (bundle_name, entry) in data.entries.iter() {
      for (version, metadata) in entry.iter() {
        let item = ListBundleManifestItem {
          name: bundle_name.to_string(),
          version: version.to_string(),
          metadata: metadata.clone(),
        };
        items.push(item);
      }
    }
    Ok(items)
  }

  // pub async fn load_current_version_with_metadata(
  //   &self,
  //   bundle_name: &str,
  // ) -> crate::Result<Option<(String, BundleManifestMetadata)>> {
  //   let data = self.load().await?.read().await;
  //   let current_version = data
  //     .entries
  //     .get(bundle_name)
  //     .map(|entry| entry.current_version.to_string());
  //   // We can now safely get the metadata, since manifest data ensure the current version has
  //   // metadata in versions field.
  //   if let Some(ver) = current_version {
  //     let metadata = data
  //       .entries
  //       .get(bundle_name)
  //       .map(|entry| entry.versions.get(&ver).cloned())
  //       .flatten()
  //       .unwrap();
  //     return Ok(Some((ver, metadata)));
  //   }
  //   Ok(None)
  // }

  pub async fn load_metadata(
    &self,
    bundle_name: &str,
    version: &str,
  ) -> crate::Result<Option<BundleManifestMetadata>> {
    let data = self.load().await?.read().await;
    let metadata = data
      .entries
      .get(bundle_name)
      .and_then(|entry| entry.get(version))
      .cloned();
    Ok(metadata)
  }

  async fn load(&self) -> crate::Result<&RwLock<BundleManifestData>> {
    let data = self
      .data
      .get_or_try_init(|| async {
        if !tokio::fs::try_exists(&self.filepath).await? {
          return Ok::<RwLock<BundleManifestData>, crate::Error>(Default::default());
        }
        let raw = tokio::fs::read(&self.filepath).await?;
        let data: BundleManifestData = serde_json::from_slice(&raw)?;
        Ok::<RwLock<BundleManifestData>, crate::Error>(RwLock::new(data))
      })
      .await?;
    Ok(data)
  }
}

impl BundleManifest<ReadWrite> {
  pub async fn insert_entry(
    &self,
    bundle_name: &str,
    version: &str,
    metadata: BundleManifestMetadata,
  ) -> crate::Result<bool> {
    let mut inserted = true;
    let mut data = self.load().await?.write().await;
    data
      .entries
      .entry(bundle_name.to_string())
      .and_modify(|entry| {
        if entry.contains_key(version) {
          inserted = false;
        } else {
          entry.insert(version.to_string(), metadata.clone());
        }
      })
      .or_insert_with(|| HashMap::from([(version.to_string(), metadata)]));
    Ok(inserted)
  }

  pub async fn remove_entry(&self, bundle_name: &str, version: &str) -> crate::Result<bool> {
    let mut data = self.load().await?.write().await;
    if let Some(entry) = data.entries.get_mut(bundle_name) {
      return Ok(entry.remove(version).is_some());
    }
    Ok(false)
  }

  // pub async fn update_current_version(
  //   &self,
  //   bundle_name: &str,
  //   version: &str,
  // ) -> crate::Result<()> {
  //   let mut data = self.load().await?.write().await;
  //   match data.entries.get_mut(bundle_name) {
  //     Some(entry) => {
  //       if !entry.versions.contains_key(version) {
  //         return Err(crate::Error::bundle_entry_not_exists(bundle_name, version));
  //       }
  //       entry.current_version = version.to_string();
  //     }
  //     None => {
  //       return Err(crate::Error::bundle_entry_not_exists(bundle_name, version));
  //     }
  //   }
  //   Ok(())
  // }

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
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::testing::*;
  use std::sync::Arc;

  #[tokio::test]
  async fn list_entries() {
    let fixtures = Fixtures::new();
    let manifest = BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadOnly,
    );
    let items = manifest.list_entries().await.unwrap();
    assert_eq!(items.len(), 2);
    let current = items
      .iter()
      .find(|x| x.name == "nextjs" && x.current)
      .unwrap();
    assert_eq!(current.version, "1.0.0");
  }

  #[tokio::test]
  async fn load_metadata() {
    let fixtures = Fixtures::new();
    let manifest = BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadOnly,
    );
    let metadata = manifest
      .load_metadata("nextjs", "1.0.0")
      .await
      .unwrap()
      .unwrap();
    assert!(manifest
      .load_metadata("nextjs", "not_exists")
      .await
      .unwrap()
      .is_none());
  }

  #[tokio::test]
  async fn load_metadata_many_times() {
    let fixtures = Fixtures::new();
    let manifest = Arc::new(BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadOnly,
    ));
    let mut handlers = vec![];
    for _ in 1..10 {
      let m = manifest.clone();
      let handle = tokio::spawn(async move { m.load_metadata("nextjs", "1.0.0").await });
      handlers.push(handle);
    }
    for h in handlers {
      h.await.unwrap().unwrap();
    }
  }

  #[tokio::test]
  async fn load_current_version() {
    let fixtures = Fixtures::new();
    let manifest = BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadOnly,
    );
    let version = manifest
      .load_current_version("nextjs")
      .await
      .unwrap()
      .unwrap();
    assert_eq!(version, "1.0.0");
  }

  #[tokio::test]
  async fn load_current_version_many_times() {
    let fixtures = Fixtures::new();
    let manifest = Arc::new(BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadOnly,
    ));
    let mut handlers = vec![];
    for _ in 1..10 {
      let m = manifest.clone();
      let handle = tokio::spawn(async move { m.load_current_version("nextjs").await });
      handlers.push(handle);
    }
    for h in handlers {
      let version = h.await.unwrap().unwrap().unwrap();
      assert_eq!(version, "1.0.0");
    }
  }

  #[tokio::test]
  async fn update_current_version() {
    let fixtures = Fixtures::new();
    let manifest = BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadWrite,
    );
    manifest
      .update_current_version("nextjs", "1.1.0")
      .await
      .unwrap();
    assert_eq!(
      manifest
        .load_current_version("nextjs")
        .await
        .unwrap()
        .unwrap(),
      "1.1.0"
    );
  }

  #[tokio::test]
  async fn update_current_version_entry_not_exists() {
    let fixtures = Fixtures::new();
    let manifest = BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadWrite,
    );
    let err = manifest
      .update_current_version("nextjs", "not_exists")
      .await
      .unwrap_err();
    assert_eq!(
      err.to_string(),
      "bundle entry not exists (bundle_name: nextjs, version: not_exists)"
    );
    let err = manifest
      .update_current_version("not_exists", "1.0.0")
      .await
      .unwrap_err();
    assert_eq!(
      err.to_string(),
      "bundle entry not exists (bundle_name: not_exists, version: 1.0.0)"
    );
  }

  #[tokio::test]
  async fn insert_entry() {
    let fixtures = Fixtures::new();
    let manifest = BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadWrite,
    );
    let metadata = BundleManifestMetadata {
      etag: None,
      integrity: None,
      signature: None,
      last_modified: 0,
    };
    let inserted = manifest
      .insert_entry("nextjs", "1.2.0", metadata.clone())
      .await
      .unwrap();
    assert!(inserted);
    assert_eq!(
      manifest
        .load_metadata("nextjs", "1.2.0")
        .await
        .unwrap()
        .unwrap(),
      metadata
    );
  }

  #[tokio::test]
  async fn insert_entry_from_empty() {
    let fixtures = Fixtures::new();
    let manifest = BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadWrite,
    );
    let metadata = BundleManifestMetadata {
      etag: None,
      integrity: None,
      signature: None,
      last_modified: 0,
    };
    let inserted = manifest
      .insert_entry("vite", "1.0.0", metadata.clone())
      .await
      .unwrap();
    assert!(inserted);
    assert_eq!(
      manifest
        .load_metadata("vite", "1.0.0")
        .await
        .unwrap()
        .unwrap(),
      metadata
    );
    assert_eq!(
      manifest
        .load_current_version("vite")
        .await
        .unwrap()
        .unwrap(),
      "1.0.0"
    );
  }

  #[tokio::test]
  async fn remove_entry() {
    let fixtures = Fixtures::new();
    let manifest = BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadWrite,
    );
    let removed = manifest.remove_entry("nextjs", "1.1.0").await.unwrap();
    assert!(removed);
    assert!(manifest
      .load_metadata("nextjs", "1.1.0")
      .await
      .unwrap()
      .is_none());
  }

  #[tokio::test]
  async fn remove_entry_builtin_cannot_be_removed() {
    let fixtures = Fixtures::new();
    let manifest = BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadWrite,
    );
    let err = manifest.remove_entry("nextjs", "1.0.0").await.unwrap_err();
    assert_eq!(
      err.to_string(),
      "bundle cannot be removed (bundle_name: nextjs, version: 1.0.0): builtin bundle cannot be removed"
    );
  }

  #[tokio::test]
  async fn remove_entry_current_version_cannot_be_removed() {
    let fixtures = Fixtures::new();
    let manifest = BundleManifest::new(
      &fixtures.get_path("bundles").join("manifest.json"),
      ReadWrite,
    );
    manifest
      .update_current_version("nextjs", "1.1.0")
      .await
      .unwrap();
    let err = manifest.remove_entry("nextjs", "1.1.0").await.unwrap_err();
    assert_eq!(
      err.to_string(),
      "bundle cannot be removed (bundle_name: nextjs, version: 1.1.0): current version of bundle cannot be removed"
    );
  }
}
