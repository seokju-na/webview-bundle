#[cfg(feature = "remote")]
use crate::remote::RemoteBundleInfo;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum BundleMetadataVersion {
  #[default]
  V1 = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BundleMetadata {
  pub metadata_version: BundleMetadataVersion,
  pub name: String,
  pub version: String,
  pub etag: Option<String>,
  pub integrity: Option<String>,
  pub signature: Option<String>,
}

pub trait BundleMetadataFrom<T> {
  fn from(metadata_version: BundleMetadataVersion, value: T) -> Self;
}

#[cfg(feature = "remote")]
impl BundleMetadataFrom<RemoteBundleInfo> for BundleMetadata {
  fn from(metadata_version: BundleMetadataVersion, value: RemoteBundleInfo) -> Self {
    Self {
      metadata_version,
      name: value.name,
      version: value.version,
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
    }
  }
}

impl BundleMetadata {
  pub fn new(
    metadata_version: BundleMetadataVersion,
    name: impl Into<String>,
    version: impl Into<String>,
    etag: Option<impl Into<String>>,
    integrity: Option<impl Into<String>>,
    signature: Option<impl Into<String>>,
  ) -> Self {
    match metadata_version {
      BundleMetadataVersion::V1 => Self::new_v1(name, version, etag, integrity, signature),
    }
  }

  pub fn new_v1(
    name: impl Into<String>,
    version: impl Into<String>,
    etag: Option<impl Into<String>>,
    integrity: Option<impl Into<String>>,
    signature: Option<impl Into<String>>,
  ) -> Self {
    Self {
      metadata_version: BundleMetadataVersion::V1,
      name: name.into(),
      version: version.into(),
      etag: etag.map(|v| v.into()),
      integrity: integrity.map(|v| v.into()),
      signature: signature.map(|v| v.into()),
    }
  }

  pub async fn load(filepath: &Path) -> crate::Result<Self> {
    let raw = tokio::fs::read(filepath).await?;
    let metadata: Self = serde_json::from_slice(&raw)?;
    Ok(metadata)
  }

  pub async fn save(&self, filepath: &Path) -> crate::Result<()> {
    let raw = serde_json::to_vec(self)?;
    tokio::fs::write(filepath, raw).await?;
    Ok(())
  }
}
