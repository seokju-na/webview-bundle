use crate::http::request;
use crate::http::HttpMethod;
use crate::http::HttpResponse;
use crate::source::BundleSource;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;
use wvb::protocol;
use wvb::protocol::Protocol;

#[napi]
pub struct BundleProtocol {
  pub(crate) inner: Arc<protocol::BundleProtocol>,
}

#[napi]
impl BundleProtocol {
  #[napi(constructor)]
  pub fn new(source: &BundleSource) -> BundleProtocol {
    Self {
      inner: Arc::new(protocol::BundleProtocol::new(source.inner.clone())),
    }
  }

  #[napi]
  pub fn handle(
    &self,
    env: Env,
    method: HttpMethod,
    uri: String,
    headers: Option<HashMap<String, String>>,
  ) -> crate::Result<AsyncBlock<HttpResponse>> {
    let req = request(method, uri, headers)?;
    let inner = self.inner.clone();
    let resp = AsyncBlockBuilder::new(async move {
      inner
        .handle(req)
        .await
        .map(HttpResponse::from)
        .map_err(crate::Error::Core)
        .map_err(|e| e.into())
    })
    .build(&env)?;
    Ok(resp)
  }
}

#[napi]
pub struct LocalProtocol {
  pub(crate) inner: Arc<protocol::LocalProtocol>,
}

#[napi]
impl LocalProtocol {
  #[napi(constructor)]
  pub fn new(hosts: HashMap<String, String>) -> LocalProtocol {
    Self {
      inner: Arc::new(protocol::LocalProtocol::new(hosts)),
    }
  }

  #[napi]
  pub fn handle(
    &self,
    env: Env,
    method: HttpMethod,
    uri: String,
    headers: Option<HashMap<String, String>>,
  ) -> crate::Result<AsyncBlock<HttpResponse>> {
    let req = request(method, uri, headers)?;
    let inner = self.inner.clone();
    let resp = AsyncBlockBuilder::new(async move {
      inner
        .handle(req)
        .await
        .map(HttpResponse::from)
        .map_err(crate::Error::Core)
        .map_err(|e| e.into())
    })
    .build(&env)?;
    Ok(resp)
  }
}
