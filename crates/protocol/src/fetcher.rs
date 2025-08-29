use webview_bundle::{Bundle, BundleManifest};

pub trait Fetcher {
  fn fetch(&self) -> crate::Result<Bundle>;
  fn fetch_manifest(&self) -> crate::Result<BundleManifest>;
}
