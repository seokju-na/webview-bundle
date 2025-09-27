#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("core error: {0}")]
  Core(#[from] webview_bundle::Error),
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("invalid config: {0}")]
  InvalidConfig(String),
  #[error("remote bundle not found: {0}")]
  RemoteBundleNotFund(String),
  #[error("remote bundle name not exists")]
  RemoteBundleNameNotExists,
  #[error("remote bundle version not exists")]
  RemoteBundleVersionNotExists,
  #[error("remote http error with status {status}")]
  RemoteHttp { status: u16 },
  #[cfg(feature = "_opendal")]
  #[error("opendal error: {0}")]
  Opendal(#[from] opendal::Error),
}

impl Error {
  pub(crate) fn invalid_config(message: impl Into<String>) -> Self {
    Self::InvalidConfig(message.into())
  }
}

pub type Result<T> = std::result::Result<T, Error>;
