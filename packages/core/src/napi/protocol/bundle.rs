use crate::napi::http::request;
use crate::napi::http::{JsHttpMethod, JsHttpResponse};
use crate::napi::source::JsBundleSource;
use crate::protocol::BundleProtocol;
use crate::protocol::Protocol;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;

#[napi(js_name = "BundleProtocol")]
pub struct JsBundleProtocol {
  pub(crate) inner: Arc<BundleProtocol>,
}

#[napi]
impl JsBundleProtocol {
  #[napi(constructor)]
  pub fn new(source: &JsBundleSource) -> JsBundleProtocol {
    Self {
      inner: Arc::new(BundleProtocol::new(source.inner.clone())),
    }
  }

  #[napi]
  pub fn handle(
    &self,
    env: Env,
    method: JsHttpMethod,
    uri: String,
    headers: Option<HashMap<String, String>>,
  ) -> crate::Result<AsyncBlock<JsHttpResponse>> {
    let req = request(method, uri, headers)?;
    let inner = self.inner.clone();
    let resp = AsyncBlockBuilder::new(async move {
      inner
        .handle(req)
        .await
        .map(JsHttpResponse::from)
        .map_err(Error::from)
    })
    .build(&env)?;
    Ok(resp)
  }
}
