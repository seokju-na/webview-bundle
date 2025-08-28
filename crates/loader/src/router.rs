use webview_bundle::http::Uri;
use webview_bundle::BundleResponse;

pub trait Router {
  async fn route(&self, uri: &Uri) -> Result<BundleResponse, String>;
}
