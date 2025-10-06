use crate::napi::http::JsHttpHeaders;
use crate::napi::version::JsVersion;
use crate::{
  AsyncBundleReader, AsyncBundleWriter, AsyncReader, AsyncWriter, Bundle, BundleBuilder,
  BundleBuilderOptions, BundleManifest, Header, HeaderWriterOptions, Index, IndexEntry,
  IndexWriterOptions,
};
use http::HeaderMap;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::ops::Deref;
use tokio::fs;

#[napi(js_name = "Header")]
pub struct JsHeader {
  pub(crate) inner: SharedReference<JsBundleManifest, &'static Header>,
}

#[napi]
impl JsHeader {
  #[napi]
  pub fn version(&self) -> JsVersion {
    JsVersion::from(self.inner.version())
  }

  #[napi]
  pub fn index_end_offset(&self) -> u64 {
    self.inner.index_end_offset()
  }

  #[napi]
  pub fn index_size(&self) -> u32 {
    self.inner.index_size()
  }
}

#[napi(object, js_name = "IndexEntry")]
pub struct JsIndexEntry {
  pub offset: u32,
  pub len: u32,
  pub is_empty: bool,
  pub headers: HashMap<String, String>,
}

impl From<&IndexEntry> for JsIndexEntry {
  fn from(value: &IndexEntry) -> Self {
    Self {
      offset: value.offset() as u32,
      len: value.len() as u32,
      is_empty: value.is_empty(),
      headers: JsHttpHeaders::from(value.headers()).0,
    }
  }
}

#[napi(js_name = "Index")]
pub struct JsIndex {
  pub(crate) inner: SharedReference<JsBundleManifest, &'static Index>,
}

#[napi]
impl JsIndex {
  #[napi]
  pub fn entries(&self) -> HashMap<String, JsIndexEntry> {
    let mut entries = HashMap::with_capacity(self.inner.entries().len());
    for (key, value) in self.inner.entries() {
      entries.insert(key.to_string(), JsIndexEntry::from(value));
    }
    entries
  }

  #[napi]
  pub fn get_entry(&self, path: String) -> Option<JsIndexEntry> {
    self.inner.get_entry(&path).map(JsIndexEntry::from)
  }

  #[napi]
  pub fn contains_path(&self, path: String) -> bool {
    self.inner.contains_path(&path)
  }
}

pub(crate) enum JsBundleManifestInner {
  Owned(BundleManifest),
  Bundle(SharedReference<JsBundle, &'static BundleManifest>),
}

unsafe impl Send for JsBundleManifestInner {}
unsafe impl Sync for JsBundleManifestInner {}

impl Deref for JsBundleManifestInner {
  type Target = BundleManifest;
  fn deref(&self) -> &Self::Target {
    match self {
      Self::Owned(x) => x,
      Self::Bundle(x) => x,
    }
  }
}

#[napi(js_name = "BundleManifest")]
pub struct JsBundleManifest {
  pub(crate) inner: JsBundleManifestInner,
}

#[napi]
impl JsBundleManifest {
  #[napi]
  pub fn header(&self, this: Reference<JsBundleManifest>, env: Env) -> crate::Result<JsHeader> {
    let inner = this.share_with(env, |manifest| Ok(manifest.inner.header()))?;
    Ok(JsHeader { inner })
  }

  #[napi]
  pub fn index(&self, this: Reference<JsBundleManifest>, env: Env) -> crate::Result<JsIndex> {
    let inner = this.share_with(env, |manifest| Ok(manifest.inner.index()))?;
    Ok(JsIndex { inner })
  }
}

#[napi(js_name = "Bundle")]
pub struct JsBundle {
  pub(crate) inner: Bundle,
}

#[napi]
impl JsBundle {
  #[napi]
  pub fn manifest(&self, this: Reference<JsBundle>, env: Env) -> crate::Result<JsBundleManifest> {
    let inner = this.share_with(env, |bundle| Ok(bundle.inner.manifest()))?;
    Ok(JsBundleManifest {
      inner: JsBundleManifestInner::Bundle(inner),
    })
  }

  #[napi]
  pub fn get_data(&self, path: String) -> crate::Result<Option<Buffer>> {
    let buf = self.inner.get_data(&path)?.map(|x| x.into());
    Ok(buf)
  }

  #[napi]
  pub fn get_data_checksum(&self, path: String) -> crate::Result<Option<u32>> {
    let checksum = self.inner.get_data_checksum(&path)?;
    Ok(checksum)
  }
}

#[napi]
pub async fn read_bundle(filepath: String) -> crate::Result<JsBundle> {
  let mut file = fs::File::open(&filepath).await?;
  let bundle = AsyncReader::<Bundle>::read(&mut AsyncBundleReader::new(&mut file)).await?;
  Ok(JsBundle { inner: bundle })
}

#[napi]
pub async fn write_bundle(bundle: &JsBundle, filepath: String) -> crate::Result<usize> {
  let mut file = fs::File::create(&filepath).await?;
  let size =
    AsyncWriter::<Bundle>::write(&mut AsyncBundleWriter::new(&mut file), &bundle.inner).await?;
  Ok(size)
}

#[napi(object, js_name = "BuildOptions")]
pub struct JsBuildOptions {
  pub header: Option<JsBuildHeaderOptions>,
  pub index: Option<JsBuildIndexOptions>,
  pub data_checksum_seed: Option<u32>,
}

impl From<JsBuildOptions> for BundleBuilderOptions {
  fn from(value: JsBuildOptions) -> Self {
    let mut options = BundleBuilderOptions::new();
    if let Some(header) = value.header {
      options.header(header.into());
    }
    if let Some(index) = value.index {
      options.index(index.into());
    }
    if let Some(seed) = value.data_checksum_seed {
      options.data_checksum_seed(seed);
    }
    options
  }
}

#[napi(object, js_name = "BuildHeaderOptions")]
pub struct JsBuildHeaderOptions {
  pub checksum_seed: Option<u32>,
}

impl From<JsBuildHeaderOptions> for HeaderWriterOptions {
  fn from(value: JsBuildHeaderOptions) -> Self {
    let mut options = HeaderWriterOptions::new();
    if let Some(seed) = value.checksum_seed {
      options.checksum_seed(seed);
    }
    options
  }
}

#[napi(object, js_name = "BuildIndexOptions")]
pub struct JsBuildIndexOptions {
  pub checksum_seed: Option<u32>,
}

impl From<JsBuildIndexOptions> for IndexWriterOptions {
  fn from(value: JsBuildIndexOptions) -> Self {
    let mut options = IndexWriterOptions::new();
    if let Some(seed) = value.checksum_seed {
      options.checksum_seed(seed);
    }
    options
  }
}

#[napi(js_name = "BundleBuilder")]
pub struct JsNapiBundleBuilder {
  pub(crate) version: JsVersion,
  pub(crate) inner: BundleBuilder,
}

#[napi]
impl JsNapiBundleBuilder {
  #[napi(constructor)]
  pub fn new(version: Option<JsVersion>) -> JsNapiBundleBuilder {
    Self {
      version: version.unwrap_or(JsVersion::V1),
      inner: BundleBuilder::new(),
    }
  }

  #[napi(getter)]
  pub fn version(&self) -> &JsVersion {
    &self.version
  }

  #[napi]
  pub fn entry_paths(&self) -> Vec<String> {
    self.inner.entries().keys().map(|s| s.to_string()).collect()
  }

  #[napi]
  pub fn insert_entry(
    &mut self,
    path: String,
    data: Buffer,
    headers: Option<HashMap<String, String>>,
  ) -> crate::Result<bool> {
    let headers = if let Some(h) = headers {
      Some(
        HeaderMap::try_from(JsHttpHeaders::from(h))
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
      )
    } else {
      None
    };
    Ok(
      self
        .inner
        .insert_entry(path, (data.as_ref(), headers))
        .is_some(),
    )
  }

  #[napi]
  pub fn remove_entry(&mut self, path: String) -> bool {
    self.inner.remove_entry(&path).is_some()
  }

  #[napi]
  pub fn contains_entry(&self, path: String) -> bool {
    self.inner.contains_path(&path)
  }

  #[napi]
  pub fn build(&mut self, options: Option<JsBuildOptions>) -> crate::Result<JsBundle> {
    if let Some(options) = options {
      self.inner.set_options(options.into());
    }
    let bundle = self.inner.build()?;
    Ok(JsBundle { inner: bundle })
  }
}
