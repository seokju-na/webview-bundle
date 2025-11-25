#[cfg(feature = "remote")]
use crate::remote::RemoteBundleInfo;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BundleManifestVersion {
  #[serde(rename = "v1")]
  #[default]
  V1,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BundleManifestJson {
  pub manifest_version: BundleManifestVersion,
  pub name: String,
  pub version: String,
  pub etag: Option<String>,
  pub integrity: Option<String>,
  pub signature: Option<String>,
}

#[cfg(feature = "remote")]
impl From<RemoteBundleInfo> for BundleManifestJson {
  fn from(info: RemoteBundleInfo) -> Self {
    Self {
      manifest_version: BundleManifestVersion::default(),
      name: info.name,
      version: info.version,
      etag: info.etag,
      integrity: info.integrity,
      signature: info.signature,
    }
  }
}

pub struct BundleManifest {
  filepath: PathBuf,
  json: RwLock<BundleManifestJson>,
}

impl BundleManifest {
  pub async fn load(filepath: &Path) -> crate::Result<Self> {
    let filepath = filepath.to_path_buf();
    let raw = tokio::fs::read(&filepath).await?;
    let json: BundleManifestJson = serde_json::from_slice(&raw)?;
    Ok(Self {
      filepath,
      json: RwLock::new(json),
    })
  }

  pub async fn filepath(&self) -> &Path {
    &self.filepath
  }

  pub async fn save(&self) -> crate::Result<()> {
    let raw = {
      let json = self.json.read().unwrap();
      serde_json::to_vec(&*json)
    }?;
    tokio::fs::write(&self.filepath, raw).await?;
    Ok(())
  }
}
