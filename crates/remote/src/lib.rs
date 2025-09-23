mod builder;
mod config;
mod error;
#[cfg(feature = "github")]
pub mod github;

use async_trait::async_trait;
pub use builder::*;
pub use config::*;
pub use error::*;
use std::path::Path;
use webview_bundle::Bundle;

#[async_trait]
pub trait Remote: Send + Sync + Unpin + 'static {
  async fn upload(&self, name: &str, version: &str, bundle: &Bundle) -> Result<()>;
  async fn download(&self, name: &str, version: &str) -> Result<Bundle>;
}
