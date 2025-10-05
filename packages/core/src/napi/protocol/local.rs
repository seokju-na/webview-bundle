use crate::napi::http::JsHttpMethod;
use crate::napi::http::{request, JsHttpResponse};
use crate::protocol::{LocalProtocol, Protocol};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;

#[napi(js_name = "LocalProtocol")]
pub struct JsLocalProtocol {
  pub(crate) inner: Arc<LocalProtocol>,
}

#[napi]
impl JsLocalProtocol {
  #[napi(constructor)]
  pub fn new(hosts: HashMap<String, String>) -> JsLocalProtocol {
    Self {
      inner: Arc::new(LocalProtocol::new(hosts)),
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
