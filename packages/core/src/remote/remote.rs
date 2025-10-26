use crate::remote::HttpConfig;
use crate::{Bundle, BundleReader, Reader};
use futures_util::StreamExt;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteBundleInfo {
  pub name: String,
  pub version: String,
  pub integrity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteError {
  pub message: Option<String>,
}

type OnDownload = dyn Fn(u64, u64) + Send + Sync + 'static;

#[derive(Default, Clone)]
#[non_exhaustive]
pub struct RemoteConfig {
  pub endpoint: String,
  pub on_download: Option<Arc<OnDownload>>,
  pub http: Option<HttpConfig>,
}

#[derive(Default, Clone)]
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
  pub async fn list_bundles(&self) -> crate::Result<Vec<String>> {
    let resp = self.client.get(self.api_url("/bundles")).send().await?;
    match resp.status().is_success() {
      true => Ok(resp.json::<Vec<String>>().await?),
      false => Err(self.parse_err(resp).await),
    }
  }

  /// HEAD /bundles/:name
  pub async fn get_current_info(&self, bundle_name: &str) -> crate::Result<RemoteBundleInfo> {
    let resp = self
      .client
      .head(self.api_url(format!("/bundles/{bundle_name}")))
      .send()
      .await?;
    match resp.status().is_success() {
      true => Ok(self.parse_info(&resp)?),
      false => Err(self.parse_err(resp).await),
    }
  }

  /// GET /bundles/:name
  pub async fn download(
    &self,
    bundle_name: &str,
  ) -> crate::Result<(RemoteBundleInfo, Bundle, Vec<u8>)> {
    self.download_inner(format!("/bundles/{bundle_name}")).await
  }

  /// GET /bundles/:name/:version
  pub async fn download_version(
    &self,
    bundle_name: &str,
    version: &str,
  ) -> crate::Result<(RemoteBundleInfo, Bundle, Vec<u8>)> {
    self
      .download_inner(format!("/bundles/{bundle_name}/{version}"))
      .await
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

  fn parse_info(&self, resp: &reqwest::Response) -> crate::Result<RemoteBundleInfo> {
    let headers = resp.headers();
    let name = String::from_utf8_lossy(
      headers
        .get("webview-bundle-name")
        .ok_or(crate::Error::invalid_remote_bundle(
          "\"webview-bundle-name\" header is missing",
        ))?
        .as_bytes(),
    )
    .to_string();
    let version = String::from_utf8_lossy(
      headers
        .get("webview-bundle-version")
        .ok_or(crate::Error::invalid_remote_bundle(
          "\"webview-bundle-version\" header is missing",
        ))?
        .as_bytes(),
    )
    .to_string();
    let integrity = headers
      .get("webview-bundle-integrity")
      .map(|x| String::from_utf8_lossy(x.as_bytes()).to_string());
    Ok(RemoteBundleInfo {
      name,
      version,
      integrity,
    })
  }

  async fn parse_err(&self, resp: reqwest::Response) -> crate::Error {
    let status = resp.status();
    if status == StatusCode::FORBIDDEN {
      return crate::Error::RemoteForbidden;
    } else if status == StatusCode::NOT_FOUND {
      return crate::Error::RemoteBundleNotFound;
    }
    let message = resp
      .json::<RemoteError>()
      .await
      .map(|x| x.message)
      .unwrap_or_default();
    crate::Error::remote_http(status, message)
  }

  async fn download_inner(
    &self,
    path: String,
  ) -> crate::Result<(RemoteBundleInfo, Bundle, Vec<u8>)> {
    let resp = self.client.get(self.api_url(path)).send().await?;
    if !resp.status().is_success() {
      return Err(self.parse_err(resp).await);
    }
    let info = self.parse_info(&resp)?;
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
    let mut reader = Cursor::new(&data);
    let bundle = Reader::<Bundle>::read(&mut BundleReader::new(&mut reader))?;
    Ok((info, bundle, data))
  }
}
