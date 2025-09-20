use serde::Deserialize;
use std::collections::HashMap;
use tauri::utils::config::WindowConfig;

#[derive(Clone, Debug)]
pub enum ProtocolConfig {
  Bundle {
    scheme: String,
    dir: Option<String>,
    base_dir: Option<String>,
  },
  Local {
    scheme: String,
    hosts: HashMap<String, String>,
  },
}

impl ProtocolConfig {
  pub fn scheme(&self) -> &str {
    match self {
      ProtocolConfig::Bundle { scheme, .. } => scheme,
      ProtocolConfig::Local { scheme, .. } => scheme,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Config {
  pub protocols: Vec<ProtocolConfig>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonConfig {
  pub windows: Vec<WindowConfig>,
}
