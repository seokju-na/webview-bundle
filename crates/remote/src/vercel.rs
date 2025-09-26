use crate::{Builder, Config, HttpConfig, Remote, EXTENSION, MIME_TYPE};
use async_trait::async_trait;
use std::io::Cursor;
use webview_bundle::{Bundle, BundleReader, BundleWriter, Reader, Writer};

type NameResolver = dyn Fn(&str, &str) -> String + Send + Sync + 'static;

#[derive(Default)]
#[non_exhaustive]
pub struct VercelConfig {
  pub token: String,
  pub read_concurrent: Option<usize>,
  pub read_chunk: Option<usize>,
  pub write_concurrent: Option<usize>,
  pub write_chunk: Option<usize>,
  pub cache_control: Option<String>,
  pub http: Option<HttpConfig>,
  pub path: Option<Box<NameResolver>>,
  pub download: Option<Box<NameResolver>>,
}

fn default_path(name: &str, version: &str) -> String {
  // TODO: normalize bundle name
  format!("bundles/{name}/{version}/{name}_{version}{EXTENSION}")
}

impl VercelConfig {
  pub(crate) fn path(&self, bundle_name: &str, version: &str) -> String {
    match self.path.as_ref() {
      Some(path) => path(bundle_name, version),
      None => default_path(bundle_name, version),
    }
  }

  pub(crate) fn download(&self, bundle_name: &str, version: &str) -> Option<String> {
    self.download.as_ref().map(|x| x(bundle_name, version))
  }
}

impl Config for VercelConfig {
  type Builder = VercelBuilder;

  fn into_builder(self) -> Self::Builder {
    VercelBuilder { config: self }
  }
}

#[derive(Default)]
pub struct VercelBuilder {
  config: VercelConfig,
}

impl Builder for VercelBuilder {
  type Config = VercelConfig;

  fn build(self) -> crate::Result<impl Remote> {
    let mut s = opendal::services::VercelBlob::default().token(&self.config.token);
    let mut op = opendal::Operator::new(s)
      .map_err(|e| {
        if e.kind() == opendal::ErrorKind::ConfigInvalid {
          return crate::Error::invalid_config(e.to_string());
        }
        crate::Error::Opendal(e)
      })?
      .finish();
    if let Some(ref http_config) = self.config.http {
      let mut http = reqwest::ClientBuilder::new();
      http = http_config.apply_into(http);
      let client = http.build()?;
      op = op.layer(opendal::layers::HttpClientLayer::new(
        opendal::raw::HttpClient::with(client),
      ));
    }
    Ok(Vercel {
      config: self.config,
      op,
    })
  }
}

pub struct Vercel {
  config: VercelConfig,
  op: opendal::Operator,
}

#[async_trait]
impl Remote for Vercel {
  async fn upload(&self, bundle_name: &str, version: &str, bundle: &Bundle) -> crate::Result<()> {
    let path = self.config.path(bundle_name, version);
    let mut writer = self.op.writer_with(&path).content_type(MIME_TYPE);
    if let Some(concurrent) = self.config.write_concurrent {
      writer = writer.concurrent(concurrent);
    }
    if let Some(chunk) = self.config.write_chunk {
      writer = writer.chunk(chunk);
    }
    if let Some(ref download) = self.config.download {
      let download_name = download(bundle_name, version);
      let content_disposition = format!(
        "attachment; filename*=UTF-8''{}",
        urlencoding::encode(&download_name)
      );
      writer = writer.content_disposition(&content_disposition);
    }
    if let Some(ref cache_control) = self.config.cache_control {
      writer = writer.cache_control(cache_control);
    }
    let mut w = writer.await?;
    let mut data_binary = vec![];
    BundleWriter::new(&mut data_binary).write(bundle)?;
    w.write(data_binary).await?;
    w.close().await?;
    Ok(())
  }

  async fn download(&self, bundle_name: &str, version: &str) -> crate::Result<Bundle> {
    let path = self.config.path(bundle_name, version);
    let mut r = self.op.read_with(&path);
    if let Some(concurrent) = self.config.read_concurrent {
      r = r.concurrent(concurrent);
    }
    if let Some(chunk) = self.config.read_chunk {
      r = r.chunk(chunk);
    }
    let buf = r.await.map_err(|e| {
      if e.kind() == opendal::ErrorKind::NotFound {
        return crate::Error::remote_bundle_not_found(e.to_string());
      }
      crate::Error::Opendal(e)
    })?;
    let mut reader = Cursor::new(buf.to_vec());
    let bundle = Reader::<Bundle>::read(&mut BundleReader::new(&mut reader))?;
    Ok(bundle)
  }
}
