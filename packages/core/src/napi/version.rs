use napi_derive::napi;

#[napi(string_enum = "lowercase", js_name = "Version")]
pub enum JsVersion {
  V1,
}

impl From<JsVersion> for crate::Version {
  fn from(value: JsVersion) -> Self {
    match value {
      JsVersion::V1 => crate::Version::V1,
    }
  }
}

impl From<crate::Version> for JsVersion {
  fn from(value: crate::Version) -> Self {
    match value {
      crate::Version::V1 => JsVersion::V1,
    }
  }
}
