use crate::http::HttpHeaders;
use crate::version::Version;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::io::Cursor;
use std::ops::Deref;
use tokio::fs;
use webview_bundle::http::HeaderMap;
use webview_bundle::{
  AsyncBundleReader, AsyncBundleWriter, AsyncReader, AsyncWriter, BundleBuilderOptions,
  BundleReader, BundleWriter, HeaderWriterOptions, IndexWriterOptions, Reader, Writer,
};

#[napi]
pub struct Header {
  pub(crate) inner: SharedReference<BundleDescriptor, &'static webview_bundle::Header>,
}

#[napi]
impl Header {
  #[napi]
  pub fn version(&self) -> Version {
    Version::from(self.inner.version())
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

#[napi(object)]
pub struct IndexEntry {
  pub offset: u32,
  pub len: u32,
  pub is_empty: bool,
  pub headers: HashMap<String, String>,
}

impl From<&webview_bundle::IndexEntry> for IndexEntry {
  fn from(value: &webview_bundle::IndexEntry) -> Self {
    Self {
      offset: value.offset() as u32,
      len: value.len() as u32,
      is_empty: value.is_empty(),
      headers: HttpHeaders::from(value.headers()).0,
    }
  }
}

#[napi]
pub struct Index {
  pub(crate) inner: SharedReference<BundleDescriptor, &'static webview_bundle::Index>,
}

#[napi]
impl Index {
  #[napi]
  pub fn entries(&self) -> HashMap<String, IndexEntry> {
    let mut entries = HashMap::with_capacity(self.inner.entries().len());
    for (key, value) in self.inner.entries() {
      entries.insert(key.to_string(), IndexEntry::from(value));
    }
    entries
  }

  #[napi]
  pub fn get_entry(&self, path: String) -> Option<IndexEntry> {
    self.inner.get_entry(&path).map(IndexEntry::from)
  }

  #[napi]
  pub fn contains_path(&self, path: String) -> bool {
    self.inner.contains_path(&path)
  }
}

pub(crate) enum BundleDescriptorInner {
  Owned(webview_bundle::BundleDescriptor),
  Bundle(SharedReference<Bundle, &'static webview_bundle::BundleDescriptor>),
}

unsafe impl Send for BundleDescriptorInner {}
unsafe impl Sync for BundleDescriptorInner {}

impl Deref for BundleDescriptorInner {
  type Target = webview_bundle::BundleDescriptor;
  fn deref(&self) -> &Self::Target {
    match self {
      Self::Owned(x) => x,
      Self::Bundle(x) => x,
    }
  }
}

#[napi]
pub struct BundleDescriptor {
  pub(crate) inner: BundleDescriptorInner,
}

#[napi]
impl BundleDescriptor {
  #[napi]
  pub fn header(&self, this: Reference<BundleDescriptor>, env: Env) -> crate::Result<Header> {
    let inner = this.share_with(env, |manifest| Ok(manifest.inner.header()))?;
    Ok(Header { inner })
  }

  #[napi]
  pub fn index(&self, this: Reference<BundleDescriptor>, env: Env) -> crate::Result<Index> {
    let inner = this.share_with(env, |manifest| Ok(manifest.inner.index()))?;
    Ok(Index { inner })
  }
}

#[napi]
pub struct Bundle {
  pub(crate) inner: webview_bundle::Bundle,
}

#[napi]
impl Bundle {
  #[napi]
  pub fn descriptor(&self, this: Reference<Bundle>, env: Env) -> crate::Result<BundleDescriptor> {
    let inner = this.share_with(env, |bundle| Ok(bundle.inner.descriptor()))?;
    Ok(BundleDescriptor {
      inner: BundleDescriptorInner::Bundle(inner),
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
pub fn read_bundle_from_buffer(buffer: BufferSlice) -> crate::Result<Bundle> {
  let cursor = Cursor::new(buffer.as_ref());
  let bundle = Reader::<webview_bundle::Bundle>::read(&mut BundleReader::new(cursor))?;
  Ok(Bundle { inner: bundle })
}

#[napi]
pub async fn read_bundle(filepath: String) -> crate::Result<Bundle> {
  let mut file = fs::File::open(&filepath)
    .await
    .map_err(|e| crate::Error::Core(webview_bundle::Error::from(e)))?;
  let bundle =
    AsyncReader::<webview_bundle::Bundle>::read(&mut AsyncBundleReader::new(&mut file)).await?;
  Ok(Bundle { inner: bundle })
}

#[napi]
pub async fn write_bundle(bundle: &Bundle, filepath: String) -> crate::Result<usize> {
  let mut file = fs::File::create(&filepath)
    .await
    .map_err(|e| crate::Error::Core(webview_bundle::Error::from(e)))?;
  let size = AsyncWriter::<webview_bundle::Bundle>::write(
    &mut AsyncBundleWriter::new(&mut file),
    &bundle.inner,
  )
  .await?;
  Ok(size)
}

#[napi]
pub fn write_bundle_into_buffer(bundle: &Bundle) -> crate::Result<Buffer> {
  let mut buf = vec![];
  Writer::<webview_bundle::Bundle>::write(&mut BundleWriter::new(&mut buf), &bundle.inner)?;
  Ok(buf.into())
}

#[napi(object)]
pub struct BuildOptions {
  pub header: Option<BuildHeaderOptions>,
  pub index: Option<BuildIndexOptions>,
  pub data_checksum_seed: Option<u32>,
}

impl From<BuildOptions> for BundleBuilderOptions {
  fn from(value: BuildOptions) -> Self {
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

#[napi(object)]
pub struct BuildHeaderOptions {
  pub checksum_seed: Option<u32>,
}

impl From<BuildHeaderOptions> for HeaderWriterOptions {
  fn from(value: BuildHeaderOptions) -> Self {
    let mut options = HeaderWriterOptions::new();
    if let Some(seed) = value.checksum_seed {
      options.checksum_seed(seed);
    }
    options
  }
}

#[napi(object)]
pub struct BuildIndexOptions {
  pub checksum_seed: Option<u32>,
}

impl From<BuildIndexOptions> for IndexWriterOptions {
  fn from(value: BuildIndexOptions) -> Self {
    let mut options = IndexWriterOptions::new();
    if let Some(seed) = value.checksum_seed {
      options.checksum_seed(seed);
    }
    options
  }
}

#[napi]
pub struct BundleBuilder {
  pub(crate) version: Version,
  pub(crate) inner: webview_bundle::BundleBuilder,
}

#[napi]
impl BundleBuilder {
  #[napi(constructor)]
  pub fn new(version: Option<Version>) -> BundleBuilder {
    Self {
      version: version.unwrap_or(Version::V1),
      inner: webview_bundle::BundleBuilder::new(),
    }
  }

  #[napi(getter)]
  pub fn version(&self) -> &Version {
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
      Some(HeaderMap::try_from(HttpHeaders::from(h))?)
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
  pub fn build(&mut self, options: Option<BuildOptions>) -> crate::Result<Bundle> {
    if let Some(options) = options {
      self.inner.set_options(options.into());
    }
    let bundle = self.inner.build()?;
    Ok(Bundle { inner: bundle })
  }
}
