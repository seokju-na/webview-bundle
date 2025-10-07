use napi_derive::napi;
use webview_bundle::Version;

#[napi(string_enum = "lowercase", js_name = "Version")]
pub enum JsVersion {
  V1,
}

impl From<JsVersion> for Version {
  fn from(value: JsVersion) -> Self {
    match value {
      JsVersion::V1 => Version::V1,
    }
  }
}

impl From<Version> for JsVersion {
  fn from(value: Version) -> Self {
    match value {
      Version::V1 => JsVersion::V1,
    }
  }
}
