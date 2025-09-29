use crate::{AsyncBundleReader, AsyncReader, Bundle, BundleManifest};
use async_trait::async_trait;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncSeek};

#[async_trait]
pub trait Source: Send + Sync {
  type Reader: AsyncRead + AsyncSeek + Unpin + Send + Sync;

  async fn reader(&self, name: &str) -> crate::Result<Self::Reader>;
  async fn fetch(&self, name: &str) -> crate::Result<Bundle>;
  async fn fetch_manifest(&self, name: &str) -> crate::Result<BundleManifest>;
}

#[derive(Clone)]
pub struct FileSource {
  base_dir: PathBuf,
}

impl FileSource {
  pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
    Self {
      base_dir: base_dir.as_ref().to_path_buf(),
    }
  }

  pub fn file_path(&self, name: &str) -> PathBuf {
    self.base_dir.join(name).with_extension("wvb")
  }
}

#[async_trait]
impl Source for FileSource {
  type Reader = File;

  async fn reader(&self, name: &str) -> crate::Result<Self::Reader> {
    let file = File::open(&self.file_path(name))
      .await
      .map_err(map_io_err)?;
    Ok(file)
  }

  async fn fetch(&self, name: &str) -> crate::Result<Bundle> {
    let mut file = self.reader(name).await?;
    let bundle = AsyncReader::<Bundle>::read(&mut AsyncBundleReader::new(&mut file)).await?;
    Ok(bundle)
  }

  async fn fetch_manifest(&self, name: &str) -> crate::Result<BundleManifest> {
    let mut file = self.reader(name).await?;
    let manifest =
      AsyncReader::<BundleManifest>::read(&mut AsyncBundleReader::new(&mut file)).await?;
    Ok(manifest)
  }
}

fn map_io_err(e: io::Error) -> crate::Error {
  if e.kind() == io::ErrorKind::NotFound {
    return crate::Error::SourceBundleNotFound;
  }
  crate::Error::from(e)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn file_source_fetch() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let bundle = source.fetch("nextjs.wvb").await.unwrap();
    bundle.get_data("/index.html").unwrap().unwrap();
  }

  #[tokio::test]
  async fn file_source_fetch_manifest() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let manifest = source.fetch_manifest("nextjs.wvb").await.unwrap();
    assert!(manifest.index().contains_path("/index.html"));
    let reader = source.reader("nextjs.wvb").await.unwrap();
    manifest
      .async_get_data(reader, "/index.html")
      .await
      .unwrap()
      .unwrap();
  }

  #[tokio::test]
  async fn file_source_fetch_many_times() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let mut handles = Vec::new();
    for _i in 0..10 {
      let s = source.clone();
      let handle = tokio::spawn(async move {
        let bundle = s.fetch("nextjs.wvb").await.unwrap();
        bundle.get_data("/index.html").unwrap().unwrap();
      });
      handles.push(handle);
    }
    for h in handles {
      h.await.unwrap();
    }
  }

  #[tokio::test]
  async fn bundle_not_found() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let bundle = source.fetch("not-found.wvb").await;
    assert!(matches!(
      bundle.unwrap_err(),
      crate::Error::SourceBundleNotFound
    ));
  }
}
