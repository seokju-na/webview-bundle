use async_trait::async_trait;

#[cfg(feature = "integrity")]
mod integrity;
#[cfg(feature = "remote-uploader-s3")]
mod s3;

#[cfg(feature = "integrity")]
pub use integrity::*;
#[cfg(feature = "remote-uploader-s3")]
pub use s3::*;

#[async_trait]
pub trait Uploader: Send + Sync + Unpin + 'static {
  async fn upload_bundle(
    &self,
    bundle_name: &str,
    version: &str,
    bundle: &crate::Bundle,
  ) -> crate::Result<()>;
}
