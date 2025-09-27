use async_trait::async_trait;
use webview_bundle::Bundle;

#[cfg(feature = "uploader-s3")]
mod s3;
#[cfg(feature = "uploader-s3")]
pub use s3::*;

#[async_trait]
pub trait Uploader: Send + Sync + Unpin + 'static {
  async fn upload_bundle(
    &self,
    bundle_name: &str,
    version: &str,
    bundle: &Bundle,
  ) -> crate::Result<()>;
}
