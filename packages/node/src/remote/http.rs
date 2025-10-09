use napi_derive::napi;
use std::collections::HashMap;
use webview_bundle::http::{HeaderMap, HeaderName, HeaderValue};
use webview_bundle::remote::HttpConfig;

#[derive(Default)]
#[napi(object, js_name = "HttpOptions")]
pub struct JsHttpOptions {
  pub default_headers: Option<HashMap<String, String>>,
  pub user_agent: Option<String>,
  pub timeout: Option<u32>,
  pub read_timeout: Option<u32>,
  pub connect_timeout: Option<u32>,
  pub pool_idle_timeout: Option<u32>,
  pub pool_max_idle_per_host: Option<u32>,
  pub referer: Option<bool>,
  pub tcp_nodelay: Option<bool>,
  pub hickory_dns: Option<bool>,
}

impl TryFrom<JsHttpOptions> for HttpConfig {
  type Error = crate::Error;
  fn try_from(value: JsHttpOptions) -> Result<Self, Self::Error> {
    let mut config = HttpConfig::new();
    if let Some(default_headers) = value.default_headers {
      let mut headers = HeaderMap::with_capacity(default_headers.len());
      for (n, v) in default_headers {
        let name = HeaderName::from_bytes(n.as_bytes())?;
        let value = HeaderValue::from_bytes(v.as_bytes())?;
        headers.insert(name, value);
      }
    }
    if let Some(user_agent) = value.user_agent {
      config = config.user_agent(user_agent);
    }
    if let Some(timeout) = value.timeout {
      config = config.timeout(timeout as u64);
    }
    if let Some(read_timeout) = value.read_timeout {
      config = config.read_timeout(read_timeout as u64);
    }
    if let Some(connect_timeout) = value.connect_timeout {
      config = config.connect_timeout(connect_timeout as u64);
    }
    if let Some(pool_idle_timeout) = value.pool_idle_timeout {
      config = config.pool_idle_timeout(pool_idle_timeout as u64);
    }
    if let Some(pool_max_idle_per_host) = value.pool_max_idle_per_host {
      config = config.pool_max_idle_per_host(pool_max_idle_per_host as usize);
    }
    if let Some(referer) = value.referer {
      config = config.referer(referer);
    }
    if let Some(tcp_nodelay) = value.tcp_nodelay {
      config = config.tcp_nodelay(tcp_nodelay);
    }
    if let Some(hickory_dns) = value.hickory_dns {
      config = config.hickory_dns(hickory_dns);
    }
    Ok(config)
  }
}
