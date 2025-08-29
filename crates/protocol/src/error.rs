#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("core error: {0}")]
  Core(#[from] webview_bundle::Error),
  #[error("tokio runtime missing: {0}")]
  TokioRuntimeMissing(#[from] tokio::runtime::TryCurrentError),
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
}
