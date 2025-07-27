use crate::cache::Cache;
use crate::loader::Loader;
use webview_bundle::Bundle;

pub struct Config<L, C>
where
  L: Loader + Send + Sync,
  C: Cache<String, Bundle> + Send + Sync,
{
  loader: L,
  cache: C,
}

#[buildstructor::buildstructor]
impl<L, C> Config<L, C>
where
  L: Loader + Send + Sync,
  C: Cache<String, Bundle> + Send + Sync,
{
  #[builder]
  pub fn new(loader: L, cache: C) -> Self {
    Self { loader, cache }
  }

  pub fn loader(&self) -> &L {
    &self.loader
  }

  pub fn cache(&self) -> &C {
    &self.cache
  }
}
