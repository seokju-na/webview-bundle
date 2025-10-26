use crate::integrity::JsAlgorithm;
use crate::js::{JsCallback, JsCallbackExt};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use webview_bundle::remote::uploader::{IntegrityMaker, SignArgs, Signer};

#[napi(object, js_name = "SignArgs")]
pub struct JsSignArgs {
  pub integrity: String,
}

impl From<&SignArgs> for JsSignArgs {
  fn from(value: &SignArgs) -> Self {
    Self {
      integrity: value.integrity.to_string(),
    }
  }
}

#[napi(object, js_name = "IntegrityMaker", object_to_js = false)]
pub struct JsIntegrityMaker {
  pub algorithm: JsAlgorithm,
  pub sign: Option<JsCallback<JsSignArgs, Promise<String>>>,
}

impl From<JsIntegrityMaker> for IntegrityMaker {
  fn from(value: JsIntegrityMaker) -> Self {
    Self::new(
      value.algorithm.into(),
      value.sign.map(|cb| -> Arc<Signer> {
        Arc::new(move |args: &SignArgs| {
          let sign_fn = Arc::clone(&cb);
          let args = JsSignArgs::from(args);
          Box::pin(async move {
            let ret = sign_fn.invoke_async(args).await?.await?;
            Ok(ret)
          })
        })
      }),
    )
  }
}
