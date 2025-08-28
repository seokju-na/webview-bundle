use crate::router::router::Router;
use crate::BundleResponse;
use http::Uri;

pub struct LocalRouter {}

impl Router for LocalRouter {
  async fn route(&self, uri: &Uri) -> Result<BundleResponse, String> {
    todo!()
  }
}

// router
// - local
// - bundle
// loader : fetcher/cache, manifests, make source from uri
// fetcher : fetch bundle from source
// source
// - fs
// - http
