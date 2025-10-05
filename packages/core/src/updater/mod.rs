use crate::remote::{Remote, RemoteBundleInfo};
use crate::source::{BundleSource, BundleSourceVersion};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BundleUpdateInfo {
  pub name: String,
  pub version: String,
  pub local_version: Option<String>,
  pub is_available: bool,
  pub integrity: Option<String>,
}

impl From<&BundleUpdateInfo> for RemoteBundleInfo {
  fn from(value: &BundleUpdateInfo) -> Self {
    Self {
      name: value.name.to_string(),
      version: value.version.to_string(),
      integrity: value.integrity.clone(),
    }
  }
}

pub struct Updater {
  source: Arc<BundleSource>,
  remote: Arc<Remote>,
}

impl Updater {
  pub fn new(source: Arc<BundleSource>, remote: Arc<Remote>) -> Self {
    Self { source, remote }
  }

  pub async fn get_update_all(&self) -> crate::Result<Vec<BundleUpdateInfo>> {
    let remote_infos = self.remote.get_info_all().await?;
    let mut updates = Vec::with_capacity(remote_infos.len());
    for info in remote_infos {
      updates.push(self.to_update_info(info).await?);
    }
    Ok(updates)
  }

  pub async fn get_update(&self, bundle_name: &str) -> crate::Result<BundleUpdateInfo> {
    let remote_info = self.remote.get_info(bundle_name).await?;
    let info = self.to_update_info(remote_info).await?;
    Ok(info)
  }

  pub async fn download_update(&self, info: &BundleUpdateInfo) -> crate::Result<()> {
    let bundle = self.remote.download(&info.into()).await?;
    self
      .source
      .write_bundle(&info.name, &info.version, &bundle)
      .await?;
    Ok(())
  }

  pub async fn apply_update(&self, info: &BundleUpdateInfo) -> crate::Result<()> {
    let exists = self
      .source
      .is_exists(
        &info.name,
        &BundleSourceVersion::Remote(info.version.to_string()),
      )
      .await?;
    if !exists {
      return Err(crate::Error::BundleNotFound);
    }
    self.source.set_version(&info.name, &info.version).await?;
    self.source.unload_manifest(&info.name);
    Ok(())
  }

  async fn to_update_info(&self, info: RemoteBundleInfo) -> crate::Result<BundleUpdateInfo> {
    let local_version = self.source.get_version(&info.name).await?;
    let is_available = if let Some(ref local_ver) = local_version {
      local_ver.to_string() != info.version
    } else {
      true
    };
    Ok(BundleUpdateInfo {
      name: info.name,
      version: info.version,
      local_version: local_version.map(|x| x.to_string()),
      is_available,
      integrity: info.integrity.clone(),
    })
  }
}
