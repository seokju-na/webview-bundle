use crate::Remote;
use async_trait::async_trait;
use std::io::Cursor;
use webview_bundle::{Bundle, BundleReader, BundleWriter, Reader, Writer};

pub const EXTENSION: &str = ".wvb";
pub const MIME_TYPE: &str = "application/webview-bundle";

pub type NameResolver = dyn Fn(&str, &str) -> String + Send + Sync + 'static;

#[derive(Debug, Clone, Default)]
pub struct HttpConfig {
  pub(crate) user_agent: Option<String>,
  pub(crate) timeout: Option<u64>,
  pub(crate) read_timeout: Option<u64>,
  pub(crate) connect_timeout: Option<u64>,
  pub(crate) pool_idle_timeout: Option<u64>,
  pub(crate) pool_max_idle_per_host: Option<usize>,
  pub(crate) referer: Option<bool>,
  pub(crate) tcp_nodelay: Option<bool>,
  pub(crate) hickory_dns: Option<bool>,
}

impl HttpConfig {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
    self.user_agent = Some(user_agent.into());
    self
  }

  pub fn timeout(mut self, timeout: u64) -> Self {
    self.timeout = Some(timeout);
    self
  }

  pub fn read_timeout(mut self, read_timeout: u64) -> Self {
    self.read_timeout = Some(read_timeout);
    self
  }

  pub fn connect_timeout(mut self, connect_timeout: u64) -> Self {
    self.connect_timeout = Some(connect_timeout);
    self
  }

  pub fn pool_idle_timeout(mut self, pool_idle_timeout: u64) -> Self {
    self.pool_idle_timeout = Some(pool_idle_timeout);
    self
  }

  pub fn referer(mut self, referer: bool) -> Self {
    self.referer = Some(referer);
    self
  }

  pub fn tcp_nodelay(mut self, tcp_nodelay: bool) -> Self {
    self.tcp_nodelay = Some(tcp_nodelay);
    self
  }

  pub fn hickory_dns(mut self, hickory_dns: bool) -> Self {
    self.hickory_dns = Some(hickory_dns);
    self
  }

  pub(crate) fn apply_into(&self, mut http: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
    if let Some(user_agent) = self.user_agent.as_ref() {
      http = http.user_agent(user_agent);
    }
    if let Some(timeout) = self.timeout {
      http = http.timeout(std::time::Duration::from_millis(timeout));
    }
    if let Some(pool_idle_timeout) = self.pool_idle_timeout {
      http = http.pool_idle_timeout(std::time::Duration::from_millis(pool_idle_timeout));
    }
    if let Some(pool_max_idle_per_host) = self.pool_max_idle_per_host {
      http = http.pool_max_idle_per_host(pool_max_idle_per_host);
    }
    if let Some(referer) = self.referer {
      http = http.referer(referer);
    }
    if let Some(tcp_nodelay) = self.tcp_nodelay {
      http = http.tcp_nodelay(tcp_nodelay);
    }
    http
  }
}

#[derive(Default)]
pub(crate) struct OpendalConfig {
  pub read_concurrent: Option<usize>,
  pub read_chunk: Option<usize>,
  pub write_concurrent: Option<usize>,
  pub write_chunk: Option<usize>,
  pub cache_control: Option<String>,
  pub http: Option<HttpConfig>,
  pub path: Option<Box<NameResolver>>,
  pub download: Option<Box<NameResolver>>,
}

impl OpendalConfig {
  pub(crate) fn path(&self, bundle_name: &str, version: &str) -> String {
    match self.path.as_ref() {
      Some(path) => path(bundle_name, version),
      None => format!("bundles/{bundle_name}/{version}/{bundle_name}_{version}{EXTENSION}"),
    }
  }

  pub(crate) fn content_disposition(&self, bundle_name: &str, version: &str) -> Option<String> {
    if let Some(ref download) = self.download {
      let download_name = download(bundle_name, version);
      return Some(format!(
        "attachment; filename*=UTF-8''{}",
        urlencoding::encode(&download_name)
      ));
    }
    None
  }
}

pub(crate) struct OpendalRemote {
  config: OpendalConfig,
  op: opendal::Operator,
}

impl OpendalRemote {
  pub(crate) fn new<B: opendal::Builder>(b: B, config: OpendalConfig) -> crate::Result<Self> {
    let mut op = opendal::Operator::new(b)
      .map_err(|e| {
        if e.kind() == opendal::ErrorKind::ConfigInvalid {
          return crate::Error::invalid_config(e.to_string());
        }
        crate::Error::Opendal(e)
      })?
      .finish();
    if let Some(ref http_config) = config.http {
      let mut http = reqwest::ClientBuilder::new();
      http = http_config.apply_into(http);
      let client = http.build()?;
      op = op.layer(opendal::layers::HttpClientLayer::new(
        opendal::raw::HttpClient::with(client),
      ));
    }
    Ok(Self { config, op })
  }
}

#[async_trait]
impl Remote for OpendalRemote {
  async fn upload(&self, bundle_name: &str, version: &str, bundle: &Bundle) -> crate::Result<()> {
    let path = self.config.path(bundle_name, version);
    let mut writer = self.op.writer_with(&path).content_type(MIME_TYPE);
    if let Some(concurrent) = self.config.write_concurrent {
      writer = writer.concurrent(concurrent);
    }
    if let Some(chunk) = self.config.write_chunk {
      writer = writer.chunk(chunk);
    }
    if let Some(ref content_disposition) = self.config.content_disposition(bundle_name, version) {
      writer = writer.content_disposition(content_disposition);
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
