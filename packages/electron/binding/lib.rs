mod error;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use webview_bundle::http;

use error::Error;
use webview_bundle::http::HeaderMap;
use webview_bundle::protocol::Protocol;

type Result<T> = std::result::Result<T, Error>;

#[napi(string_enum = "lowercase")]
pub enum Method {
  GET,
  HEAD,
  OPTIONS,
  POST,
  PUT,
  PATCH,
  DELETE,
  TRACE,
  CONNECT,
}

impl From<Method> for http::Method {
  fn from(method: Method) -> Self {
    match method {
      Method::GET => Self::GET,
      Method::HEAD => Self::HEAD,
      Method::OPTIONS => Self::OPTIONS,
      Method::POST => Self::POST,
      Method::PUT => Self::PUT,
      Method::PATCH => Self::PATCH,
      Method::DELETE => Self::DELETE,
      Method::TRACE => Self::TRACE,
      Method::CONNECT => Self::CONNECT,
    }
  }
}

#[napi(object)]
pub struct Response {
  pub status: u16,
  pub headers: HashMap<String, String>,
  pub body: Buffer,
}

#[napi]
pub struct LocalProtocol {
  inner: Arc<webview_bundle::protocol::LocalProtocol>,
}

//noinspection RsCompileErrorMacro
#[napi]
impl LocalProtocol {
  #[napi]
  pub fn handle(
    &self,
    env: &Env,
    method: Method,
    uri: String,
    headers: Option<HashMap<String, String>>,
  ) -> Result<AsyncBlock<Response>> {
    let req = try_into_request(method, uri, headers)?;
    let inner = self.inner.clone();
    let result = AsyncBlockBuilder::new(async move {
      inner
        .handle(req)
        .await
        .map(Response::from)
        .map_err(crate::Error::from)
        .map_err(napi::Error::from)
    })
    .build(env)?;
    Ok(result)
  }
}

#[napi]
pub fn local_protocol(hosts: HashMap<String, String>) -> LocalProtocol {
  LocalProtocol {
    inner: Arc::new(webview_bundle::protocol::LocalProtocol::new(hosts)),
  }
}

#[napi]
pub struct BundleSource {
  inner: Arc<webview_bundle::source::BundleSource>,
}

#[napi]
pub async fn init_bundle_source(builtin_dir: String, remote_dir: String) -> Result<BundleSource> {
  let inner =
    webview_bundle::source::BundleSource::init(Path::new(&builtin_dir), Path::new(&remote_dir))
      .await?;
  Ok(BundleSource {
    inner: Arc::new(inner),
  })
}

#[napi]
pub struct BundleProtocol {
  inner: Arc<webview_bundle::protocol::BundleProtocol>,
}

//noinspection RsCompileErrorMacro
#[napi]
impl BundleProtocol {
  #[napi]
  pub fn handle(
    &self,
    env: &Env,
    method: Method,
    uri: String,
    headers: Option<HashMap<String, String>>,
  ) -> Result<AsyncBlock<Response>> {
    let req = try_into_request(method, uri, headers)?;
    let inner = self.inner.clone();
    let result = AsyncBlockBuilder::new(async move {
      inner
        .handle(req)
        .await
        .map(Response::from)
        .map_err(crate::Error::from)
        .map_err(napi::Error::from)
    })
    .build(env)?;
    Ok(result)
  }
}

fn try_into_request(
  method: Method,
  uri: String,
  headers: Option<HashMap<String, String>>,
) -> Result<http::Request<Vec<u8>>> {
  let mut req = http::Request::builder()
    .method(http::Method::from(method))
    .uri(&uri);
  if let Some(headers) = headers {
    for (key, value) in headers {
      req = req.header(key, value);
    }
  }
  let req = req.body(vec![])?;
  Ok(req)
}

impl From<http::Response<Cow<'static, [u8]>>> for Response {
  fn from(value: http::Response<Cow<'static, [u8]>>) -> Self {
    let status = value.status().as_u16();
    let headers = into_headers(value.headers());
    let body = Buffer::from(value.body().as_ref());
    Response {
      status,
      headers,
      body,
    }
  }
}

fn into_headers(headers: &HeaderMap) -> HashMap<String, String> {
  headers
    .iter()
    .map(|(k, v)| {
      let value = String::from_utf8_lossy(v.as_ref()).to_string();
      (k.to_string(), value)
    })
    .collect::<HashMap<_, _>>()
}

#[napi]
pub fn bundle_protocol(source: &BundleSource) -> BundleProtocol {
  let inner = webview_bundle::protocol::BundleProtocol::new(source.inner.clone());
  BundleProtocol {
    inner: Arc::new(inner),
  }
}
