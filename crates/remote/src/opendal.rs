use crate::{HttpConfig, EXTENSION};
use std::sync::Arc;

type PathResolver = dyn Fn(&str, &str) -> String + Send + Sync + 'static;

#[derive(Default, Clone)]
pub(crate) struct OpendalConfig {
  pub write_concurrent: Option<usize>,
  pub write_chunk: Option<usize>,
  pub cache_control: Option<String>,
  pub http: Option<HttpConfig>,
  pub path: Option<Arc<PathResolver>>,
  pub download_name: Option<Arc<PathResolver>>,
}

impl OpendalConfig {
  pub(crate) fn resolve_path(&self, bundle_name: &str, version: &str) -> String {
    match self.path.as_ref() {
      Some(path) => path(bundle_name, version),
      None => format!("bundles/{bundle_name}/{version}/{bundle_name}_{version}{EXTENSION}"),
    }
  }

  pub(crate) fn resolve_content_disposition(
    &self,
    bundle_name: &str,
    version: &str,
  ) -> Option<String> {
    if let Some(ref download_name_fn) = self.download_name {
      let download_name = download_name_fn(bundle_name, version);
      return Some(format!(
        "attachment; filename*=UTF-8''{}",
        urlencoding::encode(&download_name)
      ));
    }
    None
  }
}

#[macro_export]
macro_rules! impl_opendal_config_for_builder {
  ($struct:ident) => {
    impl $struct {
      pub fn write_concurrent(mut self, concurrent: usize) -> Self {
        self.config.opendal.write_concurrent = Some(concurrent);
        self
      }

      pub fn write_chunk(mut self, chunk: usize) -> Self {
        self.config.opendal.write_chunk = Some(chunk);
        self
      }

      pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.config.opendal.cache_control = Some(cache_control.into());
        self
      }

      pub fn http(mut self, http: HttpConfig) -> Self {
        self.config.opendal.http = Some(http);
        self
      }

      pub fn path<T>(mut self, path: T) -> Self
      where
        T: Fn(&str, &str) -> String + Send + Sync + 'static,
      {
        self.config.opendal.path = Some(Arc::new(path));
        self
      }

      pub fn download_name<T>(mut self, download_name: T) -> Self
      where
        T: Fn(&str, &str) -> String + Send + Sync + 'static,
      {
        self.config.opendal.download_name = Some(Arc::new(download_name));
        self
      }
    }
  };
}
