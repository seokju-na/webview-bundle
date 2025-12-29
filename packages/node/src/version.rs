use napi_derive::napi;

#[napi(string_enum = "lowercase")]
pub enum Version {
  V1,
}

impl From<Version> for webview_bundle::Version {
  fn from(value: Version) -> Self {
    match value {
      Version::V1 => webview_bundle::Version::V1,
    }
  }
}

impl From<webview_bundle::Version> for Version {
  fn from(value: webview_bundle::Version) -> Self {
    match value {
      webview_bundle::Version::V1 => Version::V1,
    }
  }
}
