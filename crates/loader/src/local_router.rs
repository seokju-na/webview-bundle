use crate::router::Router;
use webview_bundle::http::Uri;
use webview_bundle::BundleResponse;

pub struct LocalRouter {}

impl Router for LocalRouter {
  async fn route(&self, uri: &Uri) -> Result<BundleResponse, String> {
    todo!()
  }
}
