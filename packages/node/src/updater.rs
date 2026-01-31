use crate::integrity::IntegrityPolicy;
use crate::js::{JsCallback, JsCallbackExt};
use crate::remote::{ListRemoteBundleInfo, Remote, RemoteBundleInfo};
use crate::signature::SignatureVerifier;
use crate::source::BundleSource;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use wvb::integrity::IntegrityChecker;
use wvb::updater;

#[napi(object)]
pub struct BundleUpdateInfo {
  pub name: String,
  pub version: String,
  pub local_version: Option<String>,
  pub is_available: bool,
  pub etag: Option<String>,
  pub integrity: Option<String>,
  pub signature: Option<String>,
  pub last_modified: Option<String>,
}

impl From<updater::BundleUpdateInfo> for BundleUpdateInfo {
  fn from(value: updater::BundleUpdateInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      local_version: value.local_version,
      is_available: value.is_available,
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
      last_modified: value.last_modified,
    }
  }
}

impl From<BundleUpdateInfo> for updater::BundleUpdateInfo {
  fn from(value: BundleUpdateInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      local_version: value.local_version,
      is_available: value.is_available,
      etag: value.etag,
      integrity: value.integrity,
      signature: value.signature,
      last_modified: value.last_modified,
    }
  }
}

pub(crate) type UpdateIntegrityChecker = JsCallback<FnArgs<(Buffer, String)>, Promise<bool>>;

#[napi(object, object_to_js = false)]
pub struct UpdaterOptions {
  pub channel: Option<String>,
  pub integrity_policy: Option<IntegrityPolicy>,
  #[napi(ts_type = "(data: Uint8Array, integrity: string) => Promise<boolean>")]
  pub integrity_checker: Option<UpdateIntegrityChecker>,
  #[napi(
    ts_type = "SignatureVerifierOptions | ((data: Uint8Array, signature: string) => Promise<boolean>)"
  )]
  pub signature_verifier: Option<SignatureVerifier>,
}

impl From<UpdaterOptions> for updater::UpdaterConfig {
  fn from(value: UpdaterOptions) -> Self {
    let mut config = updater::UpdaterConfig::default();
    if let Some(channel) = value.channel {
      config = config.channel(channel);
    }
    if let Some(policy) = value.integrity_policy {
      config = config.integrity_policy(policy.into());
    }
    if let Some(checker) = value.integrity_checker {
      config = config.integrity_checker(IntegrityChecker::Custom(Arc::new(
        move |data, signature| {
          let buffer = Buffer::from(data);
          let signature = signature.to_string();
          let callback = Arc::clone(&checker);
          Box::pin(async move {
            let ret = callback
              .invoke_async((buffer, signature).into())
              .await?
              .await?;
            Ok(ret)
          })
        },
      )));
    }
    if let Some(verifier) = value.signature_verifier {
      config = config.signature_verifier(verifier.inner);
    }
    config
  }
}

#[napi]
pub struct Updater {
  pub(crate) inner: updater::Updater,
}

#[napi]
impl Updater {
  #[napi(constructor)]
  pub fn new(
    source: &BundleSource,
    remote: &Remote,
    options: Option<UpdaterOptions>,
  ) -> crate::Result<Updater> {
    let source = source.inner.clone();
    let remote = remote.inner.clone();
    Ok(Updater {
      inner: updater::Updater::new(source, remote, options.map(Into::into)),
    })
  }

  #[napi]
  pub async fn list_remotes(&self) -> crate::Result<Vec<ListRemoteBundleInfo>> {
    let remotes = self
      .inner
      .list_remotes()
      .await?
      .into_iter()
      .map(ListRemoteBundleInfo::from)
      .collect::<Vec<_>>();
    Ok(remotes)
  }

  #[napi]
  pub async fn get_update(&self, bundle_name: String) -> crate::Result<BundleUpdateInfo> {
    let update = self.inner.get_update(&bundle_name).await?;
    Ok(BundleUpdateInfo::from(update))
  }

  #[napi]
  pub async fn download_update(
    &self,
    bundle_name: String,
    version: Option<String>,
  ) -> crate::Result<RemoteBundleInfo> {
    let info = self.inner.download_update(bundle_name, version).await?;
    Ok(info.into())
  }
}
