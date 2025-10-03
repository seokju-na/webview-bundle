use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Core(#[from] webview_bundle::Error),
  #[error("io error: {0}")]
  Io(#[from] io::Error),
  #[error("invalid header name")]
  InvalidHeaderName(#[from] webview_bundle::http::header::InvalidHeaderName),
  #[error("invalid header value")]
  InvalidHeaderValue(#[from] webview_bundle::http::header::InvalidHeaderValue),
}

impl From<Error> for napi::Error {
  fn from(value: Error) -> Self {
    match value {
      Error::Core(e) => napi::Error::new(napi::Status::GenericFailure, format!("core error: {e}")),
      Error::Io(e) => napi::Error::new(napi::Status::GenericFailure, format!("{e}")),
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
