use crate::js::{JsCallback, JsCallbackExt};
use napi::bindgen_prelude::{Buffer, FromNapiValue, Promise, TypeName, ValidateNapiValue};
use napi::{sys, Either};
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::integrity;

#[napi(string_enum = "camelCase")]
pub enum IntegrityAlgorithm {
  Sha256,
  Sha384,
  Sha512,
}

impl From<integrity::IntegrityAlgorithm> for IntegrityAlgorithm {
  fn from(value: integrity::IntegrityAlgorithm) -> Self {
    match value {
      integrity::IntegrityAlgorithm::Sha256 => Self::Sha256,
      integrity::IntegrityAlgorithm::Sha384 => Self::Sha384,
      integrity::IntegrityAlgorithm::Sha512 => Self::Sha512,
    }
  }
}

impl From<IntegrityAlgorithm> for integrity::IntegrityAlgorithm {
  fn from(value: IntegrityAlgorithm) -> Self {
    match value {
      IntegrityAlgorithm::Sha256 => integrity::IntegrityAlgorithm::Sha256,
      IntegrityAlgorithm::Sha384 => integrity::IntegrityAlgorithm::Sha384,
      IntegrityAlgorithm::Sha512 => integrity::IntegrityAlgorithm::Sha512,
    }
  }
}

#[napi(object, object_to_js = false)]
pub struct IntegrityMakerOptions {
  pub algorithm: Option<IntegrityAlgorithm>,
}

pub struct IntegrityMaker {
  pub(crate) inner: integrity::IntegrityMaker,
}

type NapiIntegrityMaker = Either<IntegrityMakerOptions, JsCallback<Buffer, Promise<String>>>;

impl TypeName for IntegrityMaker {
  fn type_name() -> &'static str {
    NapiIntegrityMaker::type_name()
  }

  fn value_type() -> napi::ValueType {
    NapiIntegrityMaker::value_type()
  }
}

impl ValidateNapiValue for IntegrityMaker {
  unsafe fn validate(
    env: sys::napi_env,
    napi_val: sys::napi_value,
  ) -> napi::Result<sys::napi_value> {
    unsafe { NapiIntegrityMaker::validate(env, napi_val) }
  }
}

impl FromNapiValue for IntegrityMaker {
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> napi::Result<Self> {
    unsafe {
      let value = NapiIntegrityMaker::from_napi_value(env, napi_val)?;
      let value = match value {
        Either::A(inner) => integrity::IntegrityMaker::Default(inner.algorithm.map(Into::into)),
        Either::B(inner) => integrity::IntegrityMaker::Custom(Arc::new(move |data| {
          let buffer = Buffer::from(data);
          let callback = Arc::clone(&inner);
          Box::pin(async move {
            let ret = callback.invoke_async(buffer).await?.await?;
            Ok(ret)
          })
        })),
      };
      Ok(Self { inner: value })
    }
  }
}

#[napi(string_enum = "camelCase")]
pub enum IntegrityPolicy {
  Strict,
  Optional,
  None,
}

impl From<integrity::IntegrityPolicy> for IntegrityPolicy {
  fn from(value: integrity::IntegrityPolicy) -> Self {
    match value {
      integrity::IntegrityPolicy::Strict => Self::Strict,
      integrity::IntegrityPolicy::Optional => Self::Optional,
      integrity::IntegrityPolicy::None => Self::None,
    }
  }
}

impl From<IntegrityPolicy> for integrity::IntegrityPolicy {
  fn from(value: IntegrityPolicy) -> Self {
    match value {
      IntegrityPolicy::Strict => Self::Strict,
      IntegrityPolicy::Optional => Self::Optional,
      IntegrityPolicy::None => Self::None,
    }
  }
}
