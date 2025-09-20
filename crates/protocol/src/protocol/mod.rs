mod bundle;
mod local;

use async_trait::async_trait;
use std::borrow::Cow;
use webview_bundle::http::{Request, Response};

#[async_trait]
pub trait Protocol: Send + Sync {
  async fn handle(&self, request: Request<Vec<u8>>) -> crate::Result<Response<Cow<'static, [u8]>>>;
}

pub use bundle::*;
pub use local::*;
