#![deny(clippy::all)]

mod error;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use tokio::fs;
use webview_bundle::{
  http, AsyncBundleReader, AsyncBundleWriter, AsyncReader, AsyncWriter, BundleBuilderOptions,
  HeaderWriterOptions, IndexWriterOptions,
};

type Result<T> = std::result::Result<T, error::Error>;

#[napi]
pub struct Bundle {
  inner: webview_bundle::Bundle,
}

#[napi]
impl Bundle {
  #[napi(getter)]
  pub fn version(&self) -> Version {
    Version::from(self.inner.manifest().header().version())
  }

  #[napi]
  pub fn paths(&self) -> Vec<String> {
    self
      .inner
      .manifest()
      .index()
      .entries()
      .keys()
      .map(|x| x.to_owned())
      .collect::<Vec<_>>()
  }

  #[napi]
  pub fn has_path(&self, path: String) -> bool {
    self.inner.manifest().index().contains_path(&path)
  }

  #[napi]
  pub fn get_data(&self, path: String) -> Result<Option<Uint8Array>> {
    let data = self.inner.get_data(&path)?.map(Uint8Array::from);
    Ok(data)
  }

  #[napi]
  pub fn get_headers(&self, path: String) -> Option<Vec<(String, String)>> {
    self
      .inner
      .manifest()
      .index()
      .get_entry(&path)
      .map(|x| from_headers(x.headers()))
  }
}

#[napi]
pub async fn read_bundle(path: String) -> Result<Bundle> {
  let mut file = fs::File::open(&path).await?;
  let bundle =
    AsyncReader::<webview_bundle::Bundle>::read(&mut AsyncBundleReader::new(&mut file)).await?;
  Ok(Bundle { inner: bundle })
}

#[napi]
pub async fn write_bundle(bundle: &Bundle, path: String) -> Result<u32> {
  let mut file = fs::File::create(&path).await?;
  let size = AsyncWriter::<webview_bundle::Bundle>::write(
    &mut AsyncBundleWriter::new(&mut file),
    &bundle.inner,
  )
  .await?;
  Ok(size as u32)
}

#[napi(string_enum = "lowercase")]
pub enum Version {
  V1,
}

impl From<Version> for webview_bundle::Version {
  fn from(value: Version) -> Self {
    match value {
      Version::V1 => webview_bundle::Version::V1,
    }
  }
}

impl From<webview_bundle::Version> for Version {
  fn from(value: webview_bundle::Version) -> Self {
    match value {
      webview_bundle::Version::V1 => Version::V1,
    }
  }
}

fn into_headers(value: Vec<(String, String)>) -> Result<http::HeaderMap> {
  let mut headers = http::HeaderMap::new();
  for (key, value) in value {
    let k = http::HeaderName::try_from(&key)?;
    let v = http::HeaderValue::try_from(&value)?;
    headers.append(k, v);
  }
  Ok(headers)
}

fn from_headers(headers: &http::HeaderMap) -> Vec<(String, String)> {
  let mut value = vec![];
  for (k, v) in headers {
    value.push((k.to_string(), v.to_str().unwrap().to_string()));
  }
  value
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
  version: Version,
  inner: webview_bundle::BundleBuilder,
}

//noinspection RsCompileErrorMacro
#[napi]
impl BundleBuilder {
  #[napi(constructor)]
  pub fn new(version: Option<Version>) -> BundleBuilder {
    Self {
      version: version.unwrap_or(Version::from(webview_bundle::Version::default())),
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
    headers: Option<Vec<(String, String)>>,
  ) -> Result<bool> {
    let headers = if let Some(h) = headers {
      Some(into_headers(h)?)
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
  pub fn has_entry(&self, path: String) -> bool {
    self.inner.contains_path(&path)
  }

  #[napi]
  pub fn build(&mut self, options: Option<BuildOptions>) -> Result<Bundle> {
    if let Some(options) = options {
      self.inner.set_options(options.into());
    }
    let bundle = self.inner.build()?;
    Ok(Bundle { inner: bundle })
  }
}
