use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tauri::http::Uri;
use webview_bundle::Bundle;

#[async_trait]
pub trait Loader: Send + Sync {
  type Error: std::error::Error;
  fn get_bundle_name(&self, _: &Uri) -> Option<String> {
    None
  }
  async fn load(&self, uri: &Uri) -> Result<Bundle, Self::Error>;
}

pub struct FSLoader {
  pub resolve_file_path: Box<dyn Fn(&Uri) -> PathBuf + Send + Sync>,
}

impl FSLoader {
  pub fn new<R: Fn(&Uri) -> PathBuf + Send + Sync + 'static>(resolve_file_path: R) -> Self {
    Self {
      resolve_file_path: Box::new(resolve_file_path),
    }
  }

  pub fn from_dir<P: AsRef<Path>>(dir: P) -> Self {
    let dir_path_buf = dir.as_ref().to_path_buf();
    Self::new(move |uri| {
      let host = uri.host().unwrap_or_default();
      let filename = match host.ends_with(".wvb") {
        true => host.to_string(),
        false => format!("{host}.wvb"),
      };
      let mut filepath = dir_path_buf.clone();
      filepath.push(filename);
      filepath
    })
  }
}

#[async_trait]
impl Loader for FSLoader {
  type Error = crate::error::Error;

  fn get_bundle_name(&self, uri: &Uri) -> Option<String> {
    let filepath_buf = (self.resolve_file_path)(uri);
    let filepath = Path::new(&filepath_buf);
    filepath
      .file_name()
      .map(|x| x.to_string_lossy().to_string())
  }

  async fn load(&self, uri: &Uri) -> Result<Bundle, Self::Error> {
    let filepath_buf = (self.resolve_file_path)(uri);
    let buf = tokio::fs::read(&filepath_buf).await?;
    let bundle = webview_bundle::decode(buf)?;
    Ok(bundle)
  }
}
