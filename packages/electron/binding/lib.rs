#![deny(clippy::all)]

mod error;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;
use webview_bundle::http;

use error::Error;
use webview_bundle::protocol::Protocol;
use webview_bundle::source::FileSource;

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
  #[napi(constructor)]
  pub fn new(mapping: HashMap<String, String>) -> Self {
    Self {
      inner: Arc::new(webview_bundle::protocol::LocalProtocol::new(mapping)),
    }
  }

  #[napi]
  pub fn handle(
    &self,
    env: &Env,
    method: Method,
    uri: String,
    headers: Option<HashMap<String, String>>,
  ) -> Result<AsyncBlock<Response>> {
    let mut req = http::Request::builder()
      .method(http::Method::from(method))
      .uri(&uri);
    if let Some(headers) = headers {
      for (key, value) in headers {
        req = req.header(key, value);
      }
    }
    let req = req.body(vec![])?;
    let inner = self.inner.clone();
    let result = AsyncBlockBuilder::new(async move {
      inner
        .handle(req)
        .await
        .map(|resp| {
          let status = resp.status().as_u16();
          let headers = resp
            .headers()
            .iter()
            .map(|(k, v)| {
              let value = String::from_utf8_lossy(v.as_ref()).to_string();
              (k.to_string(), value)
            })
            .collect::<HashMap<_, _>>();
          let body = Buffer::from(resp.body().as_ref());
          Response {
            status,
            headers,
            body,
          }
        })
        .map_err(crate::Error::from)
        .map_err(napi::Error::from)
    })
    .build(env)?;
    Ok(result)
  }
}

#[napi]
pub struct BundleProtocol {
  inner: Arc<webview_bundle::protocol::BundleProtocol<FileSource>>,
}

//noinspection RsCompileErrorMacro
#[napi]
impl BundleProtocol {
  #[napi(constructor)]
  pub fn new(base_dir: String) -> Self {
    Self {
      inner: Arc::new(webview_bundle::protocol::BundleProtocol::new(
        FileSource::new(base_dir),
      )),
    }
  }

  #[napi]
  pub fn handle(
    &self,
    env: &Env,
    method: Method,
    uri: String,
    headers: Option<HashMap<String, String>>,
  ) -> Result<AsyncBlock<Response>> {
    let mut req = http::Request::builder()
      .method(http::Method::from(method))
      .uri(&uri);
    if let Some(headers) = headers {
      for (key, value) in headers {
        req = req.header(key, value);
      }
    }
    let req = req.body(vec![])?;
    let inner = self.inner.clone();
    let result = AsyncBlockBuilder::new(async move {
      inner
        .handle(req)
        .await
        .map(|resp| {
          let status = resp.status().as_u16();
          let headers = resp
            .headers()
            .iter()
            .map(|(k, v)| {
              let value = String::from_utf8_lossy(v.as_ref()).to_string();
              (k.to_string(), value)
            })
            .collect::<HashMap<_, _>>();
          let body = Buffer::from(resp.body().as_ref());
          Response {
            status,
            headers,
            body,
          }
        })
        .map_err(crate::Error::from)
        .map_err(napi::Error::from)
    })
    .build(env)?;
    Ok(result)
  }
}
