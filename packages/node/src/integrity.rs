use crate::js::{JsCallback, JsCallbackExt};
use napi::bindgen_prelude::{Buffer, FromNapiValue, Promise, TypeName, ValidateNapiValue};
use napi::{sys, Either};
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::integrity::{IntegrityAlgorithm, IntegrityMaker, IntegrityPolicy};

#[napi(string_enum = "camelCase", js_name = "IntegrityAlgorithm")]
pub enum JsIntegrityAlgorithm {
  Sha256,
  Sha384,
  Sha512,
}

impl From<IntegrityAlgorithm> for JsIntegrityAlgorithm {
  fn from(value: IntegrityAlgorithm) -> Self {
    match value {
      IntegrityAlgorithm::Sha256 => Self::Sha256,
      IntegrityAlgorithm::Sha384 => Self::Sha384,
      IntegrityAlgorithm::Sha512 => Self::Sha512,
    }
  }
}

impl From<JsIntegrityAlgorithm> for IntegrityAlgorithm {
  fn from(value: JsIntegrityAlgorithm) -> Self {
    match value {
      JsIntegrityAlgorithm::Sha256 => IntegrityAlgorithm::Sha256,
      JsIntegrityAlgorithm::Sha384 => IntegrityAlgorithm::Sha384,
      JsIntegrityAlgorithm::Sha512 => IntegrityAlgorithm::Sha512,
    }
  }
}

#[napi(object, js_name = "IntegrityMakerOptions", object_to_js = false)]
pub struct JsIntegrityMakerOptions {
  pub algorithm: Option<JsIntegrityAlgorithm>,
}

pub struct JsIntegrityMaker {
  pub(crate) inner: IntegrityMaker,
}

type NapiIntegrityMaker = Either<JsIntegrityMakerOptions, JsCallback<Buffer, Promise<String>>>;

impl TypeName for JsIntegrityMaker {
  fn type_name() -> &'static str {
    NapiIntegrityMaker::type_name()
  }

  fn value_type() -> napi::ValueType {
    NapiIntegrityMaker::value_type()
  }
}

impl ValidateNapiValue for JsIntegrityMaker {
  unsafe fn validate(
    env: sys::napi_env,
    napi_val: sys::napi_value,
  ) -> napi::Result<sys::napi_value> {
    unsafe { NapiIntegrityMaker::validate(env, napi_val) }
  }
}

impl FromNapiValue for JsIntegrityMaker {
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> napi::Result<Self> {
    unsafe {
      let value = NapiIntegrityMaker::from_napi_value(env, napi_val)?;
      let value = match value {
        Either::A(inner) => IntegrityMaker::Default(inner.algorithm.map(Into::into)),
        Either::B(inner) => IntegrityMaker::Custom(Arc::new(move |data| {
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

#[napi(string_enum = "camelCase", js_name = "IntegrityPolicy")]
pub enum JsIntegrityPolicy {
  Strict,
  Optional,
  None,
}

impl From<IntegrityPolicy> for JsIntegrityPolicy {
  fn from(value: IntegrityPolicy) -> Self {
    match value {
      IntegrityPolicy::Strict => Self::Strict,
      IntegrityPolicy::Optional => Self::Optional,
      IntegrityPolicy::None => Self::None,
    }
  }
}

impl From<JsIntegrityPolicy> for IntegrityPolicy {
  fn from(value: JsIntegrityPolicy) -> Self {
    match value {
      JsIntegrityPolicy::Strict => Self::Strict,
      JsIntegrityPolicy::Optional => Self::Optional,
      JsIntegrityPolicy::None => Self::None,
    }
  }
}
