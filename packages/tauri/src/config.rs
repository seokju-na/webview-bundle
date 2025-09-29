use std::collections::HashMap;
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, Runtime};

pub(crate) const DEFAULT_DIR: &str = "bundles";
pub(crate) const DEFAULT_BASE_DIR: BaseDirectory = BaseDirectory::Resource;

#[derive(Clone, Debug)]
pub struct BundleProtocolConfig {
  scheme: String,
  dir: Option<String>,
  base_dir: Option<String>,
}

impl BundleProtocolConfig {
  pub fn new<S: Into<String>>(scheme: S) -> Self {
    Self {
      scheme: scheme.into(),
      dir: None,
      base_dir: None,
    }
  }

  pub fn new_with_dir<S: Into<String>, T: Into<String>>(scheme: S, dir: T) -> Self {
    Self {
      scheme: scheme.into(),
      dir: Some(dir.into()),
      base_dir: None,
    }
  }

  pub fn dir<T: Into<String>>(self, dir: T) -> Self {
    Self {
      dir: Some(dir.into()),
      ..self
    }
  }

  pub fn base_dir<T: Into<String>>(self, base_dir: T) -> Self {
    Self {
      base_dir: Some(base_dir.into()),
      ..self
    }
  }

  pub(crate) fn resolve_source_dir<R: Runtime>(
    &self,
    app: &AppHandle<R>,
  ) -> crate::Result<PathBuf> {
    let base_dir = self
      .base_dir
      .as_ref()
      .and_then(|x| BaseDirectory::from_variable(x))
      .unwrap_or(DEFAULT_BASE_DIR);
    let source_dir = app
      .path()
      .resolve("", base_dir)?
      .join(self.dir.as_ref().unwrap_or(&DEFAULT_DIR.to_string()));
    Ok(source_dir)
  }
}

#[derive(Clone, Debug)]
pub struct LocalProtocolConfig {
  scheme: String,
  pub(crate) hosts: HashMap<String, String>,
}

impl LocalProtocolConfig {
  pub fn new<S: Into<String>>(scheme: S) -> Self {
    Self {
      scheme: scheme.into(),
      hosts: HashMap::new(),
    }
  }

  pub fn new_with_hosts<T: Into<HashMap<String, String>>>(scheme: String, hosts: T) -> Self {
    Self {
      scheme,
      hosts: hosts.into(),
    }
  }

  pub fn host<T: Into<String>, U: Into<String>>(self, host: T, url: U) -> Self {
    let mut this = self;
    this.hosts.insert(host.into(), url.into());
    this
  }

  pub fn hosts<T: Into<HashMap<String, String>>>(self, hosts: T) -> Self {
    let mut this = self;
    this.hosts = hosts.into();
    this
  }
}

#[derive(Clone, Debug)]
pub enum Protocol {
  Bundle(BundleProtocolConfig),
  Local(LocalProtocolConfig),
}

impl Protocol {
  pub fn bundle<S: Into<String>>(scheme: S) -> BundleProtocolConfig {
    BundleProtocolConfig::new(scheme)
  }

  pub fn local<S: Into<String>>(scheme: S) -> LocalProtocolConfig {
    LocalProtocolConfig::new(scheme)
  }

  pub fn scheme(&self) -> &str {
    match self {
      Protocol::Bundle(x) => &x.scheme,
      Protocol::Local(x) => &x.scheme,
    }
  }
}

impl From<BundleProtocolConfig> for Protocol {
  fn from(value: BundleProtocolConfig) -> Self {
    Protocol::Bundle(value)
  }
}

impl From<LocalProtocolConfig> for Protocol {
  fn from(value: LocalProtocolConfig) -> Self {
    Protocol::Local(value)
  }
}

#[derive(Clone, Debug, Default)]
pub struct Config {
  pub(crate) protocols: Vec<Protocol>,
}

impl Config {
  pub fn new() -> Self {
    Self { protocols: vec![] }
  }

  pub fn protocol<P: Into<Protocol>>(self, protocol: P) -> Self {
    let mut this = self;
    this.protocols.push(protocol.into());
    this
  }
}
