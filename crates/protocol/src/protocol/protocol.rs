use std::borrow::Cow;
use webview_bundle::http::{Request, Response};

pub trait Protocol {
  fn handle(&self, request: Request<Vec<u8>>) -> crate::Result<Response<Cow<'static, [u8]>>>;
}
