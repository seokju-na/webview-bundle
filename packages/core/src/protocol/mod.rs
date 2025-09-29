mod bundle;
#[cfg(feature = "protocol-local")]
mod local;
mod mime_type;
mod uri;

use async_trait::async_trait;
use std::borrow::Cow;

#[async_trait]
pub trait Protocol: Send + Sync {
  async fn handle(
    &self,
    request: http::Request<Vec<u8>>,
  ) -> crate::Result<http::Response<Cow<'static, [u8]>>>;
}

pub use bundle::*;
#[cfg(feature = "protocol-local")]
pub use local::*;
