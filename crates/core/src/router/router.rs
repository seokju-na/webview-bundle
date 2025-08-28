use crate::BundleResponse;
use http::Uri;

pub trait Router {
  async fn route(&self, uri: &Uri) -> Result<BundleResponse, String>;
}
