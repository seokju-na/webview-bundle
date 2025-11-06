#[cfg(feature = "integrity")]
use crate::integrity::{IntegrityChecker, IntegrityPolicy};
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

#[derive(Default)]
#[non_exhaustive]
pub struct UpdaterConfig {
  #[cfg(feature = "integrity")]
  pub(crate) integrity_checker: IntegrityChecker,
  #[cfg(feature = "integrity")]
  pub(crate) integrity_policy: IntegrityPolicy,
}

impl UpdaterConfig {
  pub fn new() -> Self {
    Self::default()
  }

  #[cfg(feature = "integrity")]
  pub fn integrity_checker(mut self, checker: IntegrityChecker) -> Self {
    self.integrity_checker = checker;
    self
  }

  #[cfg(feature = "integrity")]
  pub fn integrity_policy(mut self, policy: IntegrityPolicy) -> Self {
    self.integrity_policy = policy;
    self
  }
}

pub struct Updater {
  source: Arc<BundleSource>,
  remote: Arc<Remote>,
  config: UpdaterConfig,
}

impl Updater {
  pub fn new(
    source: Arc<BundleSource>,
    remote: Arc<Remote>,
    config: Option<UpdaterConfig>,
  ) -> Self {
    Self {
      source,
      remote,
      config: config.unwrap_or_default(),
    }
  }

  pub async fn list_remotes(&self) -> crate::Result<Vec<String>> {
    self.remote.list_bundles().await
  }

  pub async fn get_update(
    &self,
    bundle_name: impl Into<String>,
  ) -> crate::Result<BundleUpdateInfo> {
    let remote_info = self.remote.get_current_info(&bundle_name.into()).await?;
    let info = self.to_update_info(remote_info).await?;
    Ok(info)
  }

  pub async fn download_update(
    &self,
    bundle_name: impl Into<String>,
    version: Option<impl Into<String>>,
  ) -> crate::Result<RemoteBundleInfo> {
    let (info, bundle, data) = match version {
      Some(ver) => {
        self
          .remote
          .download_version(&bundle_name.into(), &ver.into())
          .await
      }
      None => self.remote.download(&bundle_name.into()).await,
    }?;
    #[cfg(feature = "integrity")]
    {
      match self.config.integrity_policy {
        IntegrityPolicy::Strict | IntegrityPolicy::Optional => {
          if let Some(integrity) = &info.integrity {
            self
              .config
              .integrity_checker
              .check(integrity, &data)
              .await?;
            Ok(())
          } else if self.config.integrity_policy == IntegrityPolicy::Strict {
            Err(crate::Error::IntegrityVerifyFailed)
          } else {
            Ok(())
          }
        }
        _ => Ok(()),
      }?;
    }
    self
      .source
      .write_bundle(&info.name, &info.version, &bundle)
      .await?;
    Ok(info)
  }

  pub async fn apply_update(
    &self,
    bundle_name: impl Into<String>,
    version: impl Into<String>,
  ) -> crate::Result<()> {
    let bundle_name = bundle_name.into();
    let version = version.into();
    let exists = self
      .source
      .is_exists(
        &bundle_name,
        &BundleSourceVersion::Remote(version.to_string()),
      )
      .await?;
    if !exists {
      return Err(crate::Error::BundleNotFound);
    }
    self.source.set_version(&bundle_name, &version).await?;
    self.source.unload_manifest(&bundle_name);
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
