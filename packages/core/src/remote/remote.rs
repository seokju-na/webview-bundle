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

#[derive(Debug, Clone)]
pub struct RemoteBundle {
  pub info: RemoteBundleInfo,
  pub bundle: Bundle,
}

#[derive(Default, Clone)]
#[non_exhaustive]
pub struct RemoteConfig {
  pub endpoint: String,
  pub on_download: Option<Arc<dyn Fn(u64, u64)>>,
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

  pub async fn list_bundles(&self) -> crate::Result<Vec<String>> {
    let resp = self.client.get(self.api_url(None)).send().await?;
    match resp.status().is_success() {
      true => Ok(resp.json::<Vec<String>>().await?),
      false => Err(crate::Error::RemoteHttp {
        status: resp.status().as_u16(),
      }),
    }
  }

  pub async fn get_bundle_info(&self, bundle_name: &str) -> crate::Result<RemoteBundleInfo> {
    let api_path = format!("/{bundle_name}");
    let resp = self
      .client
      .head(self.api_url(Some(api_path)))
      .send()
      .await?;
    if !resp.status().is_success() {
      return Err(crate::Error::RemoteHttp {
        status: resp.status().as_u16(),
      });
    }
    self.parse_bundle_info(&resp)
  }

  pub async fn download_bundle(&self, bundle_name: &str) -> crate::Result<RemoteBundle> {
    let api_path = format!("/{bundle_name}");
    let resp = self.client.get(self.api_url(Some(api_path))).send().await?;
    if !resp.status().is_success() {
      return Err(crate::Error::RemoteHttp {
        status: resp.status().as_u16(),
      });
    }
    let info = self.parse_bundle_info(&resp)?;
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
    Ok(RemoteBundle { info, bundle })
  }

  fn api_url(&self, path: Option<String>) -> String {
    match path {
      Some(p) => {
        let endpoint = self
          .config
          .endpoint
          .strip_suffix('/')
          .unwrap_or(&self.config.endpoint);
        format!("{}/{}", endpoint, p.strip_prefix('/').unwrap_or(&p))
      }
      None => self.config.endpoint.to_string(),
    }
  }

  fn parse_bundle_info(&self, resp: &reqwest::Response) -> crate::Result<RemoteBundleInfo> {
    let name = resp
      .headers()
      .get("webview-bundle-name")
      .cloned()
      .map(|x| String::from_utf8_lossy(x.as_bytes()).to_string())
      .ok_or(crate::Error::RemoteBundleNameNotExists)?;
    let version = resp
      .headers()
      .get("webview-bundle-version")
      .cloned()
      .map(|x| String::from_utf8_lossy(x.as_bytes()).to_string())
      .ok_or(crate::Error::RemoteBundleVersionNotExists)?;
    let integrity = resp
      .headers()
      .get("webview-bundle-integrity")
      .cloned()
      .map(|x| String::from_utf8_lossy(x.as_bytes()).to_string());
    Ok(RemoteBundleInfo {
      name,
      version,
      integrity,
    })
  }
}
