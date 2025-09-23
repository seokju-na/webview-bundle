#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("core error: {0}")]
  Core(#[from] webview_bundle::Error),
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("invalid config: {0}")]
  InvalidConfig(String),
  #[cfg(feature = "github")]
  #[error("github error: {message}")]
  GitHub { status: u16, message: String },
}

pub type Result<T> = std::result::Result<T, Error>;
