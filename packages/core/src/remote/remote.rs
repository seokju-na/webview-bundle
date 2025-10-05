use crate::remote::HttpConfig;
use crate::{Bundle, BundleReader, Reader};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteBundleInfo {
  pub name: String,
  pub version: String,
  pub integrity: Option<String>,
}

type OnDownload = dyn Fn(u64, u64) + Send + Sync + 'static;

#[derive(Default, Clone)]
#[non_exhaustive]
pub struct RemoteConfig {
  pub endpoint: String,
  pub on_download: Option<Arc<OnDownload>>,
  pub http: Option<HttpConfig>,
}

#[derive(Default)]
pub struct RemoteBuilder {
  config: RemoteConfig,
}

impl RemoteBuilder {
  pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
    self.config.endpoint = endpoint.into();
    self
  }

  pub fn http(mut self, http: HttpConfig) -> Self {
    self.config.http = Some(http);
    self
  }

  pub fn on_download<F>(mut self, on_download: F) -> Self
  where
    F: Fn(u64, u64) + Send + Sync + 'static,
  {
    self.config.on_download = Some(Arc::new(on_download));
    self
  }

  pub fn build(self) -> crate::Result<Remote> {
    if self.config.endpoint.is_empty() {
      return Err(crate::Error::invalid_remote_config("endpoint is empty"));
    }
    let mut client_builder = reqwest::ClientBuilder::new();
    if let Some(ref http_config) = self.config.http {
      client_builder = http_config.apply(client_builder);
    }
    let client = client_builder.build()?;
    Ok(Remote {
      config: self.config,
      client,
    })
  }
}

#[derive(Clone)]
pub struct Remote {
  config: RemoteConfig,
  client: reqwest::Client,
}

impl Remote {
  pub fn builder() -> RemoteBuilder {
    RemoteBuilder::default()
  }

  /// GET /bundles
  pub async fn get_info_all(&self) -> crate::Result<Vec<RemoteBundleInfo>> {
    let resp = self.client.get(self.api_url("/bundles")).send().await?;
    match resp.status().is_success() {
      true => Ok(resp.json::<Vec<RemoteBundleInfo>>().await?),
      false => Err(crate::Error::RemoteHttp {
        status: resp.status().as_u16(),
      }),
    }
  }

  /// GET /bundles/:name
  pub async fn get_info(&self, bundle_name: &str) -> crate::Result<RemoteBundleInfo> {
    let resp = self
      .client
      .get(self.api_url(format!("/bundles/{bundle_name}")))
      .send()
      .await?;
    match resp.status().is_success() {
      true => Ok(resp.json::<RemoteBundleInfo>().await?),
      false => Err(crate::Error::RemoteHttp {
        status: resp.status().as_u16(),
      }),
    }
  }

  /// GET /bundles/:name/download/:version
  pub async fn download(&self, info: &RemoteBundleInfo) -> crate::Result<Bundle> {
    let api_path = format!("/bundles/{}/download/{}", info.name, info.version);
    let resp = self.client.get(self.api_url(api_path)).send().await?;
    if !resp.status().is_success() {
      return Err(crate::Error::RemoteHttp {
        status: resp.status().as_u16(),
      });
    }
    let total_size = resp.content_length().unwrap();
    let mut stream = resp.bytes_stream();
    let mut downloaded_bytes: u64 = 0;
    let mut data = Vec::with_capacity(total_size as usize);
    while let Some(chunk_result) = stream.next().await {
      let chunk = chunk_result?;
      data.append(&mut chunk.to_vec());
      downloaded_bytes += chunk.len() as u64;
      if let Some(on_download) = &self.config.on_download {
        on_download(downloaded_bytes, total_size);
      }
    }
    let mut reader = Cursor::new(data);
    let bundle = Reader::<Bundle>::read(&mut BundleReader::new(&mut reader))?;
    Ok(bundle)
  }

  fn api_url(&self, path: impl Into<String>) -> String {
    let endpoint = self
      .config
      .endpoint
      .strip_suffix('/')
      .unwrap_or(&self.config.endpoint);
    let p = path.into();
    format!("{}/{}", endpoint, p.strip_prefix('/').unwrap_or(&p))
  }
}
