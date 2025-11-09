use crate::integrity::JsIntegrityAlgorithm;
use crate::js::{JsCallback, JsCallbackExt};
use crate::remote::{JsRemote, JsRemoteBundleInfo};
use crate::source::JsBundleSource;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::updater::{BundleUpdateInfo, Updater};

#[napi(object, js_name = "BundleUpdateInfo")]
pub struct JsBundleUpdateInfo {
  pub name: String,
  pub version: String,
  pub local_version: Option<String>,
  pub is_available: bool,
  pub integrity: Option<String>,
  pub signature: Option<String>,
}

impl From<BundleUpdateInfo> for JsBundleUpdateInfo {
  fn from(value: BundleUpdateInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      local_version: value.local_version,
      is_available: value.is_available,
      integrity: value.integrity,
      signature: value.signature,
    }
  }
}

impl From<JsBundleUpdateInfo> for BundleUpdateInfo {
  fn from(value: JsBundleUpdateInfo) -> Self {
    Self {
      name: value.name,
      version: value.version,
      local_version: value.local_version,
      is_available: value.is_available,
      integrity: value.integrity,
      signature: value.signature,
    }
  }
}

// #[napi(object, js_name = "VerifyArgs")]
// pub struct JsVerifyArgs {
//   pub original: String,
//   pub signature: String,
// }
//
// impl From<&VerifyArgs> for JsVerifyArgs {
//   fn from(value: &VerifyArgs) -> Self {
//     Self {
//       original: value.original.to_string(),
//       signature: value.signature.to_string(),
//     }
//   }
// }
//
// #[napi(
//   object,
//   js_name = "IntegrityWithSignatureVerifier",
//   object_to_js = false
// )]
// pub struct JsIntegrityWithSignatureVerifier {
//   pub algorithm: JsIntegrityAlgorithm,
//   #[napi(ts_type = "(args: VerifierArgs) => Promise<boolean>")]
//   pub verifier: JsCallback<JsVerifyArgs, Promise<bool>>,
// }
//
// #[napi(string_enum = "lowercase", js_name = "IntegrityVerifierMode")]
// pub enum JsIntegrityVerifierMode {
//   Default,
// }
//
// impl From<JsIntegrityVerifierMode> for IntegrityVerifier {
//   fn from(value: JsIntegrityVerifierMode) -> Self {
//     match value {
//       JsIntegrityVerifierMode::Default => Self::default(),
//     }
//   }
// }
//
// #[napi(string_enum = "lowercase", js_name = "IntegrityPolicy")]
// pub enum JsIntegrityPolicy {
//   Strict,
//   Optional,
//   None,
// }

// impl From<IntegrityPolicy> for JsIntegrityPolicy {
//   fn from(value: IntegrityPolicy) -> Self {
//     match value {
//       IntegrityPolicy::Strict => Self::Strict,
//       IntegrityPolicy::Optional => Self::Optional,
//       IntegrityPolicy::None => Self::None,
//     }
//   }
// }
//
// impl From<JsIntegrityPolicy> for IntegrityPolicy {
//   fn from(value: JsIntegrityPolicy) -> Self {
//     match value {
//       JsIntegrityPolicy::Strict => Self::Strict,
//       JsIntegrityPolicy::Optional => Self::Optional,
//       JsIntegrityPolicy::None => Self::None,
//     }
//   }
// }

// #[napi(object, js_name = "UpdaterOptions", object_to_js = false)]
// #[derive(Default)]
// pub struct JsUpdaterOptions {
//   pub integrity_checker: Option<Either<JsIntegrityVerifierMode, JsIntegrityWithSignatureVerifier>>,
//   pub integrity_policy: Option<JsIntegrityPolicy>,
// }
//
// impl From<JsUpdaterOptions> for UpdaterConfig {
//   fn from(value: JsUpdaterOptions) -> Self {
//     let mut config = UpdaterConfig::new();
//     if let Some(checker) = value.integrity_checker {
//       match checker {
//         Either::A(x) => config = config.integrity_checker(x.into()),
//         Either::B(x) => {
//           config = config.integrity_checker(IntegrityVerifier::WithSignature {
//             algorithm: x.algorithm.into(),
//             verify: Arc::new(move |args: &VerifyArgs| {
//               let verify_fn = Arc::clone(&x.verifier);
//               let args = JsVerifyArgs::from(args);
//               Box::pin(async move {
//                 let ret = verify_fn.invoke_async(args).await?.await?;
//                 Ok(ret)
//               })
//             }),
//           })
//         }
//       }
//     }
//     if let Some(integrity_policy) = value.integrity_policy {
//       config = config.integrity_policy(integrity_policy.into());
//     }
//     config
//   }
// }

#[napi(js_name = "Updater")]
pub struct JsUpdater {
  pub(crate) inner: Updater,
}

#[napi]
impl JsUpdater {
  #[napi(constructor)]
  pub fn new(source: &JsBundleSource, remote: &JsRemote) -> crate::Result<JsUpdater> {
    let source = source.inner.clone();
    let remote = remote.inner.clone();
    Ok(JsUpdater {
      inner: Updater::new(source, remote, None),
    })
  }

  #[napi]
  pub async fn list_remotes(&self) -> crate::Result<Vec<String>> {
    let remotes = self.inner.list_remotes().await?;
    Ok(remotes)
  }

  #[napi]
  pub async fn get_update(&self, bundle_name: String) -> crate::Result<JsBundleUpdateInfo> {
    let update = self.inner.get_update(&bundle_name).await?;
    Ok(JsBundleUpdateInfo::from(update))
  }

  #[napi]
  pub async fn download_update(
    &self,
    bundle_name: String,
    version: Option<String>,
  ) -> crate::Result<JsRemoteBundleInfo> {
    let info = self.inner.download_update(bundle_name, version).await?;
    Ok(info.into())
  }

  #[napi]
  pub async fn apply_update(&self, bundle_name: String, version: String) -> crate::Result<()> {
    self.inner.apply_update(bundle_name, version).await?;
    Ok(())
  }
}
