#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("core error: {0}")]
  Core(#[from] webview_bundle::Error),
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("join error: {0}")]
  JoinError(#[from] tokio::task::JoinError),
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
}
