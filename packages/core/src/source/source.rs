use crate::source::{BundleManifest, BundleManifestMetadata, ReadOnly, ReadWrite};
use crate::{
  AsyncBundleReader, AsyncBundleWriter, AsyncReader, AsyncWriter, Bundle, BundleDescriptor,
  EXTENSION,
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::sync::OnceCell;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum BundleSourceKind {
  Builtin,
  Remote,
}

pub type CustomVersionSelector = dyn Fn(
    &str,
    (String, BundleManifestMetadata),
    (String, BundleManifestMetadata),
  ) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>
  + Send
  + Sync;

#[derive(Default)]
#[non_exhaustive]
pub enum BundleSourceVersionSelector {
  #[default]
  LastModified,
  SemVer,
  Custom(Arc<CustomVersionSelector>),
}

impl BundleSourceVersionSelector {
  pub fn select(
    &self,
    bundle_name: &str,
    builtin: (String, BundleManifestMetadata),
    remote: (String, BundleManifestMetadata),
  ) -> crate::Result<String> {
    match self {
      Self::LastModified => {
        let builtin_last_modified = builtin.1.last_modified;
        let remote_last_modified = remote.1.last_modified;
        if builtin_last_modified < remote_last_modified {
          return Ok(remote.0);
        }
        Ok(builtin.0)
      }
      Self::SemVer => {
        let builtin_version =
          semver::Version::parse(&builtin.0).map_err(|e| crate::Error::generic(e))?;
        let remote_version =
          semver::Version::parse(&remote.0).map_err(|e| crate::Error::generic(e))?;
        if builtin_version < remote_version {
          return Ok(remote.0);
        }
        Ok(builtin.0)
      }
      Self::Custom(f) => f(bundle_name, builtin, remote).map_err(|e| crate::Error::generic(e)),
    }
  }
}

pub struct BundleSource {
  builtin_dir: PathBuf,
  builtin_manifest: BundleManifest<ReadOnly>,
  remote_dir: PathBuf,
  remote_manifest: BundleManifest<ReadWrite>,
  version_selector: BundleSourceVersionSelector,
  use_remote_first: bool,
  descriptors: DashMap<String, OnceCell<Arc<BundleDescriptor>>>,
}

impl BundleSource {
  pub fn new(
    builtin_dir: impl Into<PathBuf>,
    remote_dir: impl Into<PathBuf>,
    version_selector: Option<BundleSourceVersionSelector>,
    force_use_remote: Option<bool>,
  ) -> Self {
    let builtin_dir = builtin_dir.into();
    let builtin_manifest = BundleManifest::new(&builtin_dir.join("manifest.json"), ReadOnly);
    let remote_dir = remote_dir.into();
    let remote_manifest = BundleManifest::new(&remote_dir.join("manifest.json"), ReadWrite);
    Self {
      builtin_dir,
      builtin_manifest,
      remote_dir,
      remote_manifest,
      version_selector: version_selector.unwrap_or_default(),
      use_remote_first: force_use_remote.unwrap_or_default(),
      descriptors: DashMap::default(),
    }
  }

  pub async fn load_version(&self, bundle_name: &str) -> crate::Result<Option<String>> {
    if self.use_remote_first {
      let version = match self
        .remote_manifest
        .load_current_version(bundle_name)
        .await?
      {
        Some(ver) => Some(ver),
        None => match self
          .builtin_manifest
          .load_current_version(bundle_name)
          .await?
        {
          Some(ver) => Some(ver),
          None => None,
        },
      };
      return Ok(version);
    }

    match tokio::try_join!(
      self
        .builtin_manifest
        .load_current_version_with_metadata(bundle_name),
      self
        .remote_manifest
        .load_current_version_with_metadata(bundle_name),
    )? {
      (Some(builtin), Some(remote)) => {
        let ver = self.version_selector.select(bundle_name, builtin, remote)?;
        Ok(Some(ver))
      }
      (Some((builtin_ver, _)), None) => Ok(Some(builtin_ver)),
      (None, Some((remote_ver, _))) => Ok(Some(remote_ver)),
      (None, None) => Ok(None),
    }
  }

  pub async fn update_version(&self, bundle_name: &str, version: &str) -> crate::Result<()> {
    self
      .remote_manifest
      .update_current_version(bundle_name, version)
      .await
  }

  pub async fn reader(&self, bundle_name: &str) -> crate::Result<File> {
    let filepath = self
      .get_filepath(bundle_name)
      .await?
      .ok_or(crate::Error::BundleNotFound)?;
    let file = File::open(filepath).await.map_err(|e| {
      if e.kind() == std::io::ErrorKind::NotFound {
        return crate::Error::BundleNotFound;
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

  pub async fn fetch_descriptor(&self, bundle_name: &str) -> crate::Result<BundleDescriptor> {
    let mut file = self.reader(bundle_name).await?;
    let manifest =
      AsyncReader::<BundleDescriptor>::read(&mut AsyncBundleReader::new(&mut file)).await?;
    Ok(manifest)
  }

  pub async fn load_descriptor(&self, bundle_name: &str) -> crate::Result<Arc<BundleDescriptor>> {
    if let Some(entry) = self.descriptors.get(bundle_name) {
      if let Some(m) = entry.get() {
        return Ok(m.clone());
      }
    }
    let descriptor_cell = {
      let entry = self.descriptors.entry(bundle_name.to_string()).or_default();
      entry.clone()
    };
    let descriptor = descriptor_cell
      .get_or_try_init(|| async {
        let d = self.fetch_descriptor(bundle_name).await?;
        Ok::<Arc<BundleDescriptor>, crate::Error>(Arc::new(d))
      })
      .await?
      .clone();
    Ok(descriptor)
  }

  pub fn unload_descriptor(&self, bundle_name: &str) -> bool {
    self.descriptors.remove(bundle_name).is_some()
  }

  pub async fn is_exists(
    &self,
    bundle_name: &str,
    version: &BundleSourceVersion,
  ) -> crate::Result<bool> {
    let filepath = self.filepath(bundle_name, version);
    let exists = tokio::fs::metadata(&filepath).await.is_ok();
    Ok(exists)
  }

  pub async fn write_bundle(
    &self,
    bundle_name: &str,
    version: &str,
    bundle: &Bundle,
  ) -> crate::Result<()> {
    let filepath = self.filepath(
      bundle_name,
      &BundleSourceVersion::Remote(version.to_string()),
    );
    let mut file = File::create(&filepath).await?;
    AsyncBundleWriter::new(&mut file).write(bundle).await?;
    Ok(())
  }

  fn filepath(&self, bundle_name: &str, version: &BundleSourceVersion) -> PathBuf {
    // TODO: normalize bundle name
    // let filename = format!("{bundle_name}_{}.{EXTENSION}", **version);
    // match version {
    //   BundleSourceVersion::Builtin(_) => self.builtin_dir.join(bundle_name).join(filename),
    //   BundleSourceVersion::Remote(_) => self.remote_dir.join(bundle_name).join(filename),
    // }
    todo!()
  }

  async fn builtin_versions(&self) -> crate::Result<()> {
    // let filepath = self.builtin_dir.join("versions.json");
    // self
    //   .builtin_versions
    //   .get_or_try_init(|| async {
    //     let versions = BundleVersions::load(&filepath, ReadOnly).await?;
    //     Ok::<Arc<BundleVersions<ReadOnly>>, crate::Error>(Arc::new(versions))
    //   })
    //   .await
    todo!()
  }

  fn remote_manifest_dir(&self, bundle_name: &str) -> PathBuf {
    // self.remote_dir.join(bundle_name)
    todo!()
  }

  // async fn remote_manifest(&self, bundle_name: &str) -> crate::Result<()> {
  //   // let filepath = self.remote_dir.join("versions.json");
  //   // self
  //   //   .remote_versions
  //   //   .get_or_try_init(|| async {
  //   //     let versions = match BundleVersions::load(&filepath, ReadWrite).await {
  //   //       Ok(x) => Ok(x),
  //   //       Err(e) => match e {
  //   //         crate::Error::Io(io_err) => {
  //   //           if io_err.kind() == std::io::ErrorKind::NotFound {
  //   //             Ok(BundleVersions::new(&filepath, ReadWrite))
  //   //           } else {
  //   //             Err(crate::Error::from(io_err))
  //   //           }
  //   //         }
  //   //         _ => Err(e),
  //   //       },
  //   //     }?;
  //   //     Ok::<Arc<BundleVersions<ReadWrite>>, crate::Error>(Arc::new(versions))
  //   //   })
  //   //   .await
  //   todo!()
  // }

  async fn list_bundles_fs(&self, dir: &Path) -> crate::Result<Vec<String>> {
    let mut entries = match tokio::fs::read_dir(dir).await {
      Ok(read_dir) => Ok(read_dir),
      Err(e) => {
        if e.kind() == std::io::ErrorKind::NotFound {
          return Ok(vec![]);
        }
        Err(e)
      }
    }?;
    let mut bundles = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
      let is_dir = entry.file_type().await.map(|x| x.is_dir()).unwrap_or(false);
      if is_dir {
        let name = entry.file_name().to_string_lossy().to_string();
        bundles.push(name);
      }
    }
    Ok(bundles)
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//
//   #[tokio::test]
//   async fn file_source_fetch() {
//     let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//       .join("tests")
//       .join("fixtures")
//       .join("bundles");
//     let source = BundleSource::new(&base_dir.join("builtin"), &base_dir.join("remote"));
//     let bundle = source.fetch("nextjs").await.unwrap();
//     bundle.get_data("/index.html").unwrap().unwrap();
//   }
//
//   #[tokio::test]
//   async fn file_source_fetch_manifest() {
//     let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//       .join("tests")
//       .join("fixtures")
//       .join("bundles");
//     let source = BundleSource::new(&base_dir.join("builtin"), &base_dir.join("remote"));
//     let descriptor = source.fetch_descriptor("nextjs").await.unwrap();
//     assert!(descriptor.index().contains_path("/index.html"));
//     let reader = source.reader("nextjs").await.unwrap();
//     descriptor
//       .async_get_data(reader, "/index.html")
//       .await
//       .unwrap()
//       .unwrap();
//   }
//
//   #[tokio::test]
//   async fn file_source_fetch_many_times() {
//     let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//       .join("tests")
//       .join("fixtures")
//       .join("bundles");
//     let source = Arc::new(BundleSource::new(
//       &base_dir.join("builtin"),
//       &base_dir.join("remote"),
//     ));
//     let mut handles = Vec::new();
//     for _i in 0..10 {
//       let s = source.clone();
//       let handle = tokio::spawn(async move {
//         let bundle = s.fetch("nextjs").await.unwrap();
//         bundle.get_data("/index.html").unwrap().unwrap();
//       });
//       handles.push(handle);
//     }
//     for h in handles {
//       h.await.unwrap();
//     }
//   }
//
//   #[tokio::test]
//   async fn source_version_not_found() {
//     let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//       .join("tests")
//       .join("fixtures")
//       .join("bundles");
//     let source = BundleSource::new(&base_dir.join("builtin"), &base_dir.join("remote"));
//     let bundle = source.fetch("not-found").await;
//     assert!(matches!(bundle.unwrap_err(), crate::Error::BundleNotFound));
//   }
//
//   #[tokio::test]
//   async fn load_many_at_once() {
//     let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//       .join("tests")
//       .join("fixtures")
//       .join("bundles");
//     let source = Arc::new(BundleSource::new(
//       &base_dir.join("builtin"),
//       &base_dir.join("remote"),
//     ));
//     let mut handles = Vec::new();
//     for _i in 0..10 {
//       let s = source.clone();
//       let handle = tokio::spawn(async move {
//         let _ = s.load_descriptor("nextjs.wvb").await;
//       });
//       handles.push(handle);
//     }
//     for h in handles {
//       h.await.unwrap();
//     }
//   }
//
//   #[tokio::test]
//   async fn load_unload_sequential() {
//     let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//       .join("tests")
//       .join("fixtures")
//       .join("bundles");
//     let source = Arc::new(BundleSource::new(
//       &base_dir.join("builtin"),
//       &base_dir.join("remote"),
//     ));
//
//     let m1 = source.load_descriptor("nextjs").await.unwrap();
//     assert!(
//       source.unload_descriptor("nextjs"),
//       "unload should remove existing entry"
//     );
//     let m2 = source.load_descriptor("nextjs").await.unwrap();
//     assert!(
//       !Arc::ptr_eq(&m1, &m2),
//       "after unload, reloading should produce a new Arc"
//     );
//
//     assert!(source.unload_descriptor("nextjs"));
//     let m3 = source.load_descriptor("nextjs").await.unwrap();
//     assert!(!Arc::ptr_eq(&m2, &m3));
//
//     assert!(source.unload_descriptor("nextjs"));
//     let m4 = source.load_descriptor("nextjs").await.unwrap();
//     assert!(!Arc::ptr_eq(&m3, &m4));
//   }
//
//   #[tokio::test]
//   async fn load_unload_concurrently() {
//     use std::sync::Arc;
//     use tokio::sync::Barrier;
//     use tokio::task::JoinSet;
//
//     let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//       .join("tests")
//       .join("fixtures")
//       .join("bundles");
//     let source = Arc::new(BundleSource::new(
//       &base_dir.join("builtin"),
//       &base_dir.join("remote"),
//     ));
//
//     // 1) initial loads. test singleflight
//     let n = 5usize;
//     let mut set = JoinSet::new();
//     for _i in 0..n {
//       let s = source.clone();
//       set.spawn(async move { s.load_descriptor("nextjs").await });
//     }
//     let mut initials = Vec::with_capacity(n);
//     while let Some(res) = set.join_next().await {
//       let v = res.unwrap().unwrap();
//       initials.push(v);
//     }
//     for m in &initials[1..] {
//       assert!(Arc::ptr_eq(&initials[0], m));
//     }
//
//     // 2) before/after barriers
//     let barrier_before_unload = Arc::new(Barrier::new(n + 1));
//     let barrier_after_unload = Arc::new(Barrier::new(n + 1));
//
//     let mut before_set = JoinSet::new();
//     for _i in 0..n {
//       let s = source.clone();
//       let before = barrier_before_unload.clone();
//       before_set.spawn(async move {
//         before.wait().await;
//         s.load_descriptor("nextjs").await
//       });
//     }
//     let mut after_set = JoinSet::new();
//     for _i in 0..n {
//       let s = source.clone();
//       let after = barrier_after_unload.clone();
//       after_set.spawn(async move {
//         after.wait().await;
//         s.load_descriptor("nextjs").await
//       });
//     }
//
//     barrier_before_unload.wait().await;
//     assert!(source.unload_descriptor("nextjs"));
//     barrier_after_unload.wait().await;
//
//     let mut before_jobs = Vec::with_capacity(n);
//     while let Some(res) = before_set.join_next().await {
//       let v = res.unwrap().unwrap();
//       before_jobs.push(v);
//     }
//     let mut after_jobs = Vec::with_capacity(n);
//     while let Some(res) = after_set.join_next().await {
//       let v = res.unwrap().unwrap();
//       after_jobs.push(v);
//     }
//     // before jobs should be same with initial loads
//     for m in &before_jobs {
//       assert!(Arc::ptr_eq(&initials[0], m));
//     }
//     // after jobs should be not same with initial loads
//     for m in &after_jobs {
//       assert!(!Arc::ptr_eq(&initials[0], m));
//     }
//     for m in &before_jobs[1..] {
//       assert!(Arc::ptr_eq(&before_jobs[0], m));
//     }
//     for m in &after_jobs[1..] {
//       assert!(Arc::ptr_eq(&after_jobs[0], m));
//     }
//   }
//
//   #[tokio::test]
//   async fn list_bundles() {
//     let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//       .join("tests")
//       .join("fixtures")
//       .join("bundles");
//     let source = BundleSource::new(&base_dir.join("builtin"), &base_dir.join("remote"));
//     let res = source.list_bundles().await.unwrap();
//     assert_eq!(res.builtin, vec!["nextjs"]);
//     assert_eq!(res.remote, vec!["nextjs"]);
//   }
// }
