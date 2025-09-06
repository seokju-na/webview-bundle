use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::sync::RwLock;
use webview_bundle::{AsyncBundleReader, AsyncReader, Bundle, BundleManifest};

pub trait Source: Send + Sync {
  async fn fetch(&self, name: &str) -> crate::Result<Bundle>;
  async fn fetch_manifest(&self, name: &str) -> crate::Result<BundleManifest>;
}

#[derive(Clone)]
pub struct FileSource {
  base_dir: PathBuf,
  files: Arc<RwLock<HashMap<PathBuf, Arc<RwLock<File>>>>>,
}

impl FileSource {
  pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
    Self {
      base_dir: base_dir.as_ref().to_path_buf(),
      files: Arc::new(RwLock::new(HashMap::new())),
    }
  }

  pub fn file_path(&self, name: &str) -> PathBuf {
    self.base_dir.join(name).with_extension("wvb")
  }

  pub async fn open_file<P: AsRef<Path>>(&self, file_path: P) -> crate::Result<File> {
    let key = file_path.as_ref().to_path_buf();
    {
      let files = self.files.read().await;
      if let Some(f) = files.get(&key) {
        let f_lock = f.read().await;
        let file = f_lock.try_clone().await?;
        return Ok(file);
      }
    }
    let mut files = self.files.write().await;
    let file = File::open(file_path).await?;
    let cloned = file.try_clone().await?;
    files.insert(key, Arc::new(RwLock::new(cloned)));
    Ok(file)
  }
}

impl Source for FileSource {
  async fn fetch(&self, name: &str) -> crate::Result<Bundle> {
    let mut file = self.open_file(&self.file_path(name)).await?;
    let bundle = AsyncReader::<Bundle>::read(&mut AsyncBundleReader::new(&mut file)).await?;
    Ok(bundle)
  }

  async fn fetch_manifest(&self, name: &str) -> crate::Result<BundleManifest> {
    let mut file = self.open_file(&self.file_path(name)).await?;
    let manifest =
      AsyncReader::<BundleManifest>::read(&mut AsyncBundleReader::new(&mut file)).await?;
    Ok(manifest)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio::io::AsyncReadExt;

  #[tokio::test]
  async fn file_source_opens() {
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let source = FileSource::new(file_path.parent().unwrap());
    let mut handles = Vec::new();
    for i in 0..10 {
      let p = file_path.clone();
      let s = source.clone();
      let handle = tokio::spawn(async move {
        let mut file = s.open_file(&p).await.unwrap();
        let mut buf = String::new();
        let size = file.read_to_string(&mut buf).await.unwrap();
        println!("{i}: {size}");
      });
      handles.push(handle);
    }
    for h in handles {
      let _ = h.await;
    }
  }

  #[tokio::test]
  async fn file_source_fetch() {}
}
