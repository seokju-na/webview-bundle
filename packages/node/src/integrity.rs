use napi_derive::napi;
use webview_bundle::integrity::Algorithm;

#[napi(string_enum = "lowercase", js_name = "Algorithm")]
pub enum JsAlgorithm {
  Sha256,
  Sha384,
  Sha512,
}

impl From<Algorithm> for JsAlgorithm {
  fn from(value: Algorithm) -> Self {
    match value {
      Algorithm::Sha256 => Self::Sha256,
      Algorithm::Sha384 => Self::Sha384,
      Algorithm::Sha512 => Self::Sha512,
    }
  }
}

impl From<JsAlgorithm> for Algorithm {
  fn from(value: JsAlgorithm) -> Self {
    match value {
      JsAlgorithm::Sha256 => Algorithm::Sha256,
      JsAlgorithm::Sha384 => Algorithm::Sha384,
      JsAlgorithm::Sha512 => Algorithm::Sha512,
    }
  }
}
