use webview_bundle::http;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Core(#[from] webview_bundle::Error),
  #[error(transparent)]
  InvalidHeaderName(#[from] http::header::InvalidHeaderName),
  #[error(transparent)]
  InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
  #[error(transparent)]
  Napi(#[from] napi::Error),
}

impl From<Error> for napi::Error {
  fn from(value: Error) -> Self {
    match value {
      Error::Core(e) => napi::Error::new(napi::Status::GenericFailure, format!("{e}")),
      Error::InvalidHeaderName(e) => napi::Error::new(napi::Status::InvalidArg, e.to_string()),
      Error::InvalidHeaderValue(e) => napi::Error::new(napi::Status::InvalidArg, e.to_string()),
      Error::Napi(e) => e,
    }
  }
}

impl From<Error> for napi::JsError {
  fn from(value: Error) -> Self {
    napi::JsError::from(napi::Error::from(value))
  }
}
