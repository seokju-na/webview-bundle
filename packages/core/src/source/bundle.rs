use crate::source::versions::{BundleVersions, ReadOnly, ReadWrite};
use crate::{AsyncBundleReader, AsyncReader, Bundle, BundleManifest, EXTENSION};
use dashmap::DashMap;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::sync::OnceCell;

pub enum BundleSourceVersion {
  Builtin(String),
  Remote(String),
}

impl Deref for BundleSourceVersion {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Builtin(v) => v,
      Self::Remote(v) => v,
    }
  }
}

pub struct BundleSource {
  builtin_dir: PathBuf,
  builtin_versions: Arc<BundleVersions<ReadOnly>>,
  remote_dir: PathBuf,
  remote_versions: Arc<BundleVersions<ReadWrite>>,
  manifests: DashMap<String, Arc<OnceCell<Arc<BundleManifest>>>>,
}

impl BundleSource {
  pub async fn init(builtin_dir: &Path, remote_dir: &Path) -> crate::Result<Self> {
    let builtin_versions =
      BundleVersions::load(&builtin_dir.join("versions.json"), ReadOnly).await?;
    let remote_version =
      match BundleVersions::load(&remote_dir.join("versions.json"), ReadWrite).await {
        Ok(x) => Ok(x),
        Err(e) => match e {
          crate::Error::Io(io_err) => {
            if io_err.kind() == std::io::ErrorKind::NotFound {
              Ok(BundleVersions::new(remote_dir, ReadWrite))
            } else {
              Err(crate::Error::from(io_err))
            }
          }
          _ => Err(e),
        },
      }?;
    Ok(Self {
      builtin_dir: builtin_dir.to_path_buf(),
      builtin_versions: Arc::new(builtin_versions),
      remote_dir: remote_dir.to_path_buf(),
      remote_versions: Arc::new(remote_version),
      manifests: DashMap::default(),
    })
  }

  pub fn get_filepath(&self, bundle_name: &str) -> Option<PathBuf> {
    self
      .get_version(bundle_name)
      .map(|version| self.filepath(bundle_name, &version))
  }

  pub fn get_version(&self, bundle_name: &str) -> Option<BundleSourceVersion> {
    self
      .remote_versions
      .get_version(bundle_name)
      .map(BundleSourceVersion::Remote)
      .or_else(|| {
        self
          .builtin_versions
          .get_version(bundle_name)
          .map(BundleSourceVersion::Builtin)
      })
  }

  pub fn set_version(&self, bundle_name: &str, version: &str) {
    self.remote_versions.set_version(bundle_name, version);
  }

  pub async fn reader(&self, bundle_name: &str) -> crate::Result<File> {
    let filepath = self
      .get_filepath(bundle_name)
      .ok_or(crate::Error::SourceBundleNotFound)?;
    let file = File::open(filepath).await.map_err(|e| {
      if e.kind() == std::io::ErrorKind::NotFound {
        return crate::Error::SourceBundleNotFound;
      }
      crate::Error::from(e)
    })?;
    Ok(file)
  }

  pub async fn fetch(&self, bundle_name: &str) -> crate::Result<Bundle> {
    let mut file = self.reader(bundle_name).await?;
    let bundle = AsyncReader::<Bundle>::read(&mut AsyncBundleReader::new(&mut file)).await?;
    Ok(bundle)
  }

  pub async fn fetch_manifest(&self, bundle_name: &str) -> crate::Result<BundleManifest> {
    let mut file = self.reader(bundle_name).await?;
    let manifest =
      AsyncReader::<BundleManifest>::read(&mut AsyncBundleReader::new(&mut file)).await?;
    Ok(manifest)
  }

  pub async fn load_manifest(&self, bundle_name: &str) -> crate::Result<Arc<BundleManifest>> {
    if let Some(entry) = self.manifests.get(bundle_name) {
      if let Some(m) = entry.get() {
        return Ok(m.clone());
      }
    }
    let cell_arc = {
      let entry = self.manifests.entry(bundle_name.to_string()).or_default();
      entry.clone()
    };
    let m = cell_arc
      .get_or_try_init(|| async {
        let m = self.fetch_manifest(bundle_name).await?;
        Ok::<Arc<BundleManifest>, crate::Error>(Arc::new(m))
      })
      .await?
      .clone();
    Ok(m)
  }

  pub fn unload_manifest(&self, bundle_name: &str) -> bool {
    self.manifests.remove(bundle_name).is_some()
  }

  fn filepath(&self, bundle_name: &str, version: &BundleSourceVersion) -> PathBuf {
    // TODO: normalize bundle name
    let filename = format!("{bundle_name}_{}.{EXTENSION}", **version);
    match version {
      BundleSourceVersion::Builtin(_) => self.builtin_dir.join(filename),
      BundleSourceVersion::Remote(_) => self.remote_dir.join(filename),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn file_source_fetch() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = BundleSource::init(&base_dir.join("builtin"), &base_dir.join("remote"))
      .await
      .unwrap();
    let bundle = source.fetch("nextjs").await.unwrap();
    bundle.get_data("/index.html").unwrap().unwrap();
  }

  #[tokio::test]
  async fn file_source_fetch_manifest() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = BundleSource::init(&base_dir.join("builtin"), &base_dir.join("remote"))
      .await
      .unwrap();
    let manifest = source.fetch_manifest("nextjs").await.unwrap();
    assert!(manifest.index().contains_path("/index.html"));
    let reader = source.reader("nextjs").await.unwrap();
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
      .join("fixtures")
      .join("bundles");
    let source = Arc::new(
      BundleSource::init(&base_dir.join("builtin"), &base_dir.join("remote"))
        .await
        .unwrap(),
    );
    let mut handles = Vec::new();
    for _i in 0..10 {
      let s = source.clone();
      let handle = tokio::spawn(async move {
        let bundle = s.fetch("nextjs").await.unwrap();
        bundle.get_data("/index.html").unwrap().unwrap();
      });
      handles.push(handle);
    }
    for h in handles {
      h.await.unwrap();
    }
  }

  #[tokio::test]
  async fn source_version_not_found() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = BundleSource::init(&base_dir.join("builtin"), &base_dir.join("remote"))
      .await
      .unwrap();
    let bundle = source.fetch("not-found").await;
    assert!(matches!(
      bundle.unwrap_err(),
      crate::Error::SourceBundleNotFound
    ));
  }

  #[tokio::test]
  async fn load_many_at_once() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = Arc::new(
      BundleSource::init(&base_dir.join("builtin"), &base_dir.join("remote"))
        .await
        .unwrap(),
    );
    let mut handles = Vec::new();
    for _i in 0..10 {
      let s = source.clone();
      let handle = tokio::spawn(async move {
        let _ = s.load_manifest("nextjs.wvb").await;
      });
      handles.push(handle);
    }
    for h in handles {
      h.await.unwrap();
    }
  }

  #[tokio::test]
  async fn load_unload_sequential() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = Arc::new(
      BundleSource::init(&base_dir.join("builtin"), &base_dir.join("remote"))
        .await
        .unwrap(),
    );

    let m1 = source.load_manifest("nextjs").await.unwrap();
    assert!(
      source.unload_manifest("nextjs"),
      "unload should remove existing entry"
    );
    let m2 = source.load_manifest("nextjs").await.unwrap();
    assert!(
      !Arc::ptr_eq(&m1, &m2),
      "after unload, reloading should produce a new Arc"
    );

    assert!(source.unload_manifest("nextjs"));
    let m3 = source.load_manifest("nextjs").await.unwrap();
    assert!(!Arc::ptr_eq(&m2, &m3));

    assert!(source.unload_manifest("nextjs"));
    let m4 = source.load_manifest("nextjs").await.unwrap();
    assert!(!Arc::ptr_eq(&m3, &m4));
  }

  #[tokio::test]
  async fn load_unload_concurrently() {
    use std::sync::Arc;
    use tokio::sync::Barrier;
    use tokio::task::JoinSet;

    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = Arc::new(
      BundleSource::init(&base_dir.join("builtin"), &base_dir.join("remote"))
        .await
        .unwrap(),
    );

    // 1) initial loads. test singleflight
    let n = 5usize;
    let mut set = JoinSet::new();
    for _i in 0..n {
      let s = source.clone();
      set.spawn(async move { s.load_manifest("nextjs").await });
    }
    let mut initials = Vec::with_capacity(n);
    while let Some(res) = set.join_next().await {
      let v = res.unwrap().unwrap();
      initials.push(v);
    }
    for m in &initials[1..] {
      assert!(Arc::ptr_eq(&initials[0], m));
    }

    // 2) before/after barriers
    let barrier_before_unload = Arc::new(Barrier::new(n + 1));
    let barrier_after_unload = Arc::new(Barrier::new(n + 1));

    let mut before_set = JoinSet::new();
    for _i in 0..n {
      let s = source.clone();
      let before = barrier_before_unload.clone();
      before_set.spawn(async move {
        before.wait().await;
        s.load_manifest("nextjs").await
      });
    }
    let mut after_set = JoinSet::new();
    for _i in 0..n {
      let s = source.clone();
      let after = barrier_after_unload.clone();
      after_set.spawn(async move {
        after.wait().await;
        s.load_manifest("nextjs").await
      });
    }

    barrier_before_unload.wait().await;
    assert!(source.unload_manifest("nextjs"));
    barrier_after_unload.wait().await;

    let mut before_jobs = Vec::with_capacity(n);
    while let Some(res) = before_set.join_next().await {
      let v = res.unwrap().unwrap();
      before_jobs.push(v);
    }
    let mut after_jobs = Vec::with_capacity(n);
    while let Some(res) = after_set.join_next().await {
      let v = res.unwrap().unwrap();
      after_jobs.push(v);
    }
    // before jobs should be same with initial loads
    for m in &before_jobs {
      assert!(Arc::ptr_eq(&initials[0], m));
    }
    // after jobs should be not same with initial loads
    for m in &after_jobs {
      assert!(!Arc::ptr_eq(&initials[0], m));
    }
    for m in &before_jobs[1..] {
      assert!(Arc::ptr_eq(&before_jobs[0], m));
    }
    for m in &after_jobs[1..] {
      assert!(Arc::ptr_eq(&after_jobs[0], m));
    }
  }
}
