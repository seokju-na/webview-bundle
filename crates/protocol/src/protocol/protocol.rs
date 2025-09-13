use std::borrow::Cow;
use webview_bundle::http::{Request, Response};

pub trait Protocol: Send + Sync {
  fn handle(
    &self,
    request: Request<Vec<u8>>,
  ) -> impl std::future::Future<Output = crate::Result<Response<Cow<'static, [u8]>>>>;
}
