use std::collections::HashMap;
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, Runtime};

type DynamicDirFn<R> = fn(app: &AppHandle<R>) -> Result<PathBuf, Box<dyn std::error::Error>>;

#[derive(Clone, Debug)]
pub(crate) enum Dir<R: Runtime> {
  Static(String),
  Dynamic(DynamicDirFn<R>),
}

impl<R: Runtime> Dir<R> {
  pub fn resolve(&self, app: &AppHandle<R>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    match self {
      Self::Static(dir) => Ok(PathBuf::from(dir)),
      Self::Dynamic(f) => {
        let dir = f(app)?;
        Ok(dir)
      }
    }
  }
}

#[derive(Clone, Default, Debug)]
pub struct Source<R: Runtime> {
  pub(crate) builtin_dir: Option<Dir<R>>,
  pub(crate) remote_dir: Option<Dir<R>>,
}

impl<R: Runtime> Source<R> {
  pub fn new() -> Self {
    Self {
      builtin_dir: None,
      remote_dir: None,
    }
  }

  pub fn builtin_dir<T: Into<String>>(mut self, dir: T) -> Self {
    self.builtin_dir = Some(Dir::Static(dir.into()));
    self
  }

  pub fn builtin_dir_fn(mut self, dir: DynamicDirFn<R>) -> Self {
    self.builtin_dir = Some(Dir::Dynamic(dir));
    self
  }

  pub fn remote_dir<T: Into<String>>(mut self, dir: T) -> Self {
    self.remote_dir = Some(Dir::Static(dir.into()));
    self
  }

  pub fn remote_dir_fn(mut self, dir: DynamicDirFn<R>) -> Self {
    self.remote_dir = Some(Dir::Dynamic(dir));
    self
  }

  pub(crate) fn resolve_builtin_dir(&self, app: &AppHandle<R>) -> crate::Result<PathBuf> {
    let dir = match self.builtin_dir {
      Some(ref builtin_dir) => {
        let dir = builtin_dir
          .resolve(app)
          .map_err(|e| crate::Error::FailToResolveDirectory(e.to_string()))?;
        dir
      }
      None => {
        let dir = app.path().resolve("bundles", BaseDirectory::Resource)?;
        dir
      }
    };
    Ok(dir)
  }

  pub(crate) fn resolve_remote_dir(&self, app: &AppHandle<R>) -> crate::Result<PathBuf> {
    let dir = match self.remote_dir {
      Some(ref remote_dir) => {
        let dir = remote_dir
          .resolve(app)
          .map_err(|e| crate::Error::FailToResolveDirectory(e.to_string()))?;
        dir
      }
      None => {
        let dir = app.path().resolve("bundles", BaseDirectory::Resource)?;
        dir
      }
    };
    Ok(dir)
  }
}

#[derive(Clone, Debug)]
pub struct BundleProtocolConfig {
  scheme: String,
}

impl BundleProtocolConfig {
  pub fn new<S: Into<String>>(scheme: S) -> Self {
    Self {
      scheme: scheme.into(),
    }
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

  pub fn host<T: Into<String>, U: Into<String>>(mut self, host: T, url: U) -> Self {
    self.hosts.insert(host.into(), url.into());
    self
  }

  pub fn hosts<T: Into<HashMap<String, String>>>(mut self, hosts: T) -> Self {
    self.hosts = hosts.into();
    self
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
pub struct Config<R: Runtime> {
  pub(crate) source: Source<R>,
  pub(crate) protocols: Vec<Protocol>,
}

impl<R: Runtime> Config<R> {
  pub fn new() -> Self {
    Self {
      source: Source::new(),
      protocols: vec![],
    }
  }

  pub fn source(mut self, source: Source<R>) -> Self {
    self.source = source;
    self
  }

  pub fn protocol<P: Into<Protocol>>(mut self, protocol: P) -> Self {
    self.protocols.push(protocol.into());
    self
  }
}
