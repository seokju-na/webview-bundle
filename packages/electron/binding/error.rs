#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  WebviewBundle(#[from] webview_bundle::Error),
  #[error("http error: {0}")]
  Http(#[from] webview_bundle::http::Error),
  #[error("napi error: {0}")]
  Napi(#[from] napi::Error),
  #[error("invalid header name")]
  InvalidHeaderName(#[from] webview_bundle::http::header::InvalidHeaderName),
  #[error("invalid header value")]
  InvalidHeaderValue(#[from] webview_bundle::http::header::InvalidHeaderValue),
}

impl From<Error> for napi::Error {
  fn from(value: Error) -> Self {
    match value {
      Error::WebviewBundle(e) => napi::Error::new(napi::Status::GenericFailure, format!("{e}")),
      Error::Http(e) => napi::Error::new(napi::Status::GenericFailure, format!("{e}")),
      Error::Napi(e) => e,
      Error::InvalidHeaderName(e) => napi::Error::new(napi::Status::GenericFailure, format!("{e}")),
      Error::InvalidHeaderValue(e) => {
        napi::Error::new(napi::Status::GenericFailure, format!("{e}"))
      }
    }
  }
}

impl From<Error> for napi::JsError {
  fn from(value: Error) -> Self {
    napi::JsError::from(napi::Error::from(value))
  }
}
