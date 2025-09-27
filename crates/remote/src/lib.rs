mod builder;
mod common;
mod config;
mod error;
#[cfg(feature = "github")]
mod github;
#[cfg(feature = "s3")]
mod s3;
#[cfg(feature = "vercel")]
mod vercel;

use async_trait::async_trait;
pub use builder::*;
pub use common::*;
pub use config::*;
pub use error::*;
use webview_bundle::Bundle;

#[cfg(feature = "github")]
pub use github::{GitHub, GitHubBuilder, GitHubConfig};

#[cfg(feature = "s3")]
pub use s3::{S3Builder, S3Config, S3};

#[cfg(feature = "vercel")]
pub use vercel::{Vercel, VercelBuilder, VercelConfig};

#[async_trait]
pub trait Remote: Send + Sync + Unpin + 'static {
  async fn upload(&self, bundle_name: &str, version: &str, bundle: &Bundle) -> Result<()>;
  async fn download(&self, bundle_name: &str, version: &str) -> Result<Bundle>;
}
