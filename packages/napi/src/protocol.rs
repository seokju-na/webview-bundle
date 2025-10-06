use crate::http::request;
use crate::http::JsHttpMethod;
use crate::http::JsHttpResponse;
use crate::source::JsBundleSource;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;
use webview_bundle::protocol;
use webview_bundle::protocol::Protocol;

#[napi(js_name = "BundleProtocol")]
pub struct JsBundleProtocol {
  pub(crate) inner: Arc<protocol::BundleProtocol>,
}

#[napi]
impl JsBundleProtocol {
  #[napi(constructor)]
  pub fn new(source: &JsBundleSource) -> JsBundleProtocol {
    Self {
      inner: Arc::new(protocol::BundleProtocol::new(source.inner.clone())),
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
        .map_err(|e| crate::Error::Core(webview_bundle::Error::from(e)))
        .map_err(|e| e.into())
    })
    .build(&env)?;
    Ok(resp)
  }
}

#[napi(js_name = "LocalProtocol")]
pub struct JsLocalProtocol {
  pub(crate) inner: Arc<protocol::LocalProtocol>,
}

#[napi]
impl JsLocalProtocol {
  #[napi(constructor)]
  pub fn new(hosts: HashMap<String, String>) -> JsLocalProtocol {
    Self {
      inner: Arc::new(protocol::LocalProtocol::new(hosts)),
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
        .map_err(|e| crate::Error::Core(webview_bundle::Error::from(e)))
        .map_err(|e| e.into())
    })
    .build(&env)?;
    Ok(resp)
  }
}
