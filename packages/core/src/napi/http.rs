use http::HeaderMap;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  InvalidHeaderName(#[from] http::header::InvalidHeaderName),
  #[error(transparent)]
  InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
}

#[napi(string_enum = "lowercase", js_name = "HttpMethod")]
pub enum JsHttpMethod {
  Get,
  Head,
  Options,
  Post,
  Put,
  Patch,
  Delete,
  Trace,
  Connect,
}

impl From<JsHttpMethod> for http::Method {
  fn from(method: JsHttpMethod) -> Self {
    match method {
      JsHttpMethod::Get => Self::GET,
      JsHttpMethod::Head => Self::HEAD,
      JsHttpMethod::Options => Self::OPTIONS,
      JsHttpMethod::Post => Self::POST,
      JsHttpMethod::Put => Self::PUT,
      JsHttpMethod::Patch => Self::PATCH,
      JsHttpMethod::Delete => Self::DELETE,
      JsHttpMethod::Trace => Self::TRACE,
      JsHttpMethod::Connect => Self::CONNECT,
    }
  }
}

pub struct JsHttpHeaders(pub HashMap<String, String>);

impl Deref for JsHttpHeaders {
  type Target = HashMap<String, String>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<HashMap<String, String>> for JsHttpHeaders {
  fn from(value: HashMap<String, String>) -> Self {
    Self(value)
  }
}

impl TryFrom<JsHttpHeaders> for HeaderMap {
  type Error = Error;
  fn try_from(value: JsHttpHeaders) -> Result<Self, Self::Error> {
    let mut headers = HeaderMap::with_capacity(value.len());
    for (n, v) in value.0 {
      let name = http::HeaderName::from_bytes(n.as_bytes())?;
      let value = http::HeaderValue::from_bytes(v.as_bytes())?;
      headers.insert(name, value);
    }
    Ok(headers)
  }
}

impl From<&HeaderMap> for JsHttpHeaders {
  fn from(value: &HeaderMap) -> Self {
    Self(
      value
        .iter()
        .map(|(k, v)| {
          let value = String::from_utf8_lossy(v.as_ref()).to_string();
          (k.to_string(), value)
        })
        .collect::<HashMap<_, _>>(),
    )
  }
}

#[napi(object, js_name = "HttpResponse")]
pub struct JsHttpResponse {
  pub status: u16,
  pub headers: HashMap<String, String>,
  pub body: Buffer,
}

impl From<http::Response<Cow<'static, [u8]>>> for JsHttpResponse {
  fn from(value: http::Response<Cow<'static, [u8]>>) -> Self {
    let status = value.status().as_u16();
    let headers = JsHttpHeaders::from(value.headers()).0;
    let body = Buffer::from(value.body().as_ref());
    JsHttpResponse {
      status,
      headers,
      body,
    }
  }
}

#[cfg(any(feature = "protocol", feature = "protocol-local"))]
pub(crate) fn request(
  method: JsHttpMethod,
  uri: String,
  headers: Option<HashMap<String, String>>,
) -> crate::Result<http::Request<Vec<u8>>> {
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
