use crate::js::{JsCallback, JsCallbackExt};
use napi::bindgen_prelude::{Buffer, FromNapiValue, Promise, TypeName, ValidateNapiValue};
use napi::{sys, Either};
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::integrity::{IntegrityAlgorithm, IntegrityMaker};

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

pub struct JsIntegrityMaker {
  pub(crate) inner: IntegrityMaker,
}

type NapiIntegrityMaker = Either<JsIntegrityAlgorithm, JsCallback<Buffer, Promise<String>>>;

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
        Either::A(inner) => IntegrityMaker::Default(Some(inner.into())),
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
