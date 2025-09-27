use crate::common::{HttpConfig, EXTENSION, MIME_TYPE};
use crate::{Builder, Config, NameResolver, Remote};
use async_trait::async_trait;
use reqwest::header;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;
use webview_bundle::{Bundle, BundleReader, BundleWriter, Reader, Writer};

#[derive(Default, Clone)]
#[non_exhaustive]
pub struct GitHubConfig {
  pub token: Option<String>,
  pub owner: String,
  pub repo: String,
  pub api_version: Option<String>,
  pub http: Option<HttpConfig>,
  pub release_tag_name: Option<Arc<NameResolver>>,
  pub release_name: Option<Arc<NameResolver>>,
  pub asset_name: Option<Arc<NameResolver>>,
}

fn default_release_name(name: &str, version: &str) -> String {
  format!("{name} v{version}")
}

fn default_asset_name(name: &str, version: &str) -> String {
  format!("{name}_v{version}{EXTENSION}")
}

fn default_tag_name(name: &str, version: &str) -> String {
  format!("{name}/{version}")
}

impl GitHubConfig {
  pub(crate) fn release_tag_name(&self, bundle_name: &str, version: &str) -> String {
    match self.release_tag_name.as_ref() {
      Some(resolver) => resolver(bundle_name, version),
      None => default_tag_name(bundle_name, version),
    }
  }

  pub(crate) fn release_name(&self, bundle_name: &str, version: &str) -> String {
    match self.release_name.as_ref() {
      Some(resolver) => resolver(bundle_name, version),
      None => default_release_name(bundle_name, version),
    }
  }

  pub(crate) fn asset_name(&self, bundle_name: &str, version: &str) -> String {
    match self.asset_name.as_ref() {
      Some(resolver) => resolver(bundle_name, version),
      None => default_asset_name(bundle_name, version),
    }
  }
}

impl Config for GitHubConfig {
  type Builder = GitHubBuilder;

  fn into_builder(self) -> Self::Builder {
    GitHubBuilder { config: self }
  }
}

#[derive(Default)]
pub struct GitHubBuilder {
  config: GitHubConfig,
}

impl GitHubBuilder {
  pub fn token(mut self, token: impl Into<String>) -> Self {
    self.config.token = Some(token.into());
    self
  }

  pub fn owner(mut self, owner: impl Into<String>) -> Self {
    self.config.owner = owner.into();
    self
  }

  pub fn repo(mut self, repo: impl Into<String>) -> Self {
    self.config.repo = repo.into();
    self
  }

  pub fn api_version(mut self, api_version: impl Into<String>) -> Self {
    self.config.api_version = Some(api_version.into());
    self
  }

  pub fn http(mut self, http: HttpConfig) -> Self {
    self.config.http = Some(http);
    self
  }

  pub fn release_tag_name<T>(mut self, resolver: T) -> Self
  where
    T: Fn(&str, &str) -> String + Send + Sync + 'static,
  {
    self.config.release_tag_name = Some(Arc::new(resolver));
    self
  }

  pub fn release_name<T>(mut self, resolver: T) -> Self
  where
    T: Fn(&str, &str) -> String + Send + Sync + 'static,
  {
    self.config.release_name = Some(Arc::new(resolver));
    self
  }

  pub fn asset_name<T>(mut self, resolver: T) -> Self
  where
    T: Fn(&str, &str) -> String + Send + Sync + 'static,
  {
    self.config.asset_name = Some(Arc::new(resolver));
    self
  }
}

const DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

impl Builder for GitHubBuilder {
  type Config = GitHubConfig;

  fn build(self) -> crate::Result<impl Remote> {
    if self.config.owner.is_empty() {
      return Err(crate::Error::invalid_config("owner is empty"));
    }
    if self.config.repo.is_empty() {
      return Err(crate::Error::invalid_config("repo is empty"));
    }
    let mut http = reqwest::ClientBuilder::new();
    let mut http_config = self.config.http.clone().unwrap_or_default();
    if http_config.user_agent.is_none() {
      http_config.user_agent = Some(DEFAULT_USER_AGENT.to_string());
    }
    http = http_config.apply_into(http);
    let mut headers = header::HeaderMap::new();
    headers.insert(
      header::ACCEPT,
      header::HeaderValue::from_static("application/vnd.github+json"),
    );
    if let Some(api_version) = self.config.api_version.as_ref() {
      headers.insert(
        header::HeaderName::from_static("X-GitHub-Api-Version"),
        header::HeaderValue::from_str(api_version)
          .map_err(|_| crate::Error::invalid_config("invalid api version"))?,
      );
    }
    if let Some(token) = self.config.token.as_ref() {
      headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(token)
          .map_err(|_| crate::Error::invalid_config("invalid token"))?,
      );
    }
    let http = http
      .default_headers(headers)
      .build()
      .map_err(|e| crate::Error::invalid_config(e.to_string()))?;
    Ok(GitHub {
      config: self.config,
      http,
    })
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum GitHubReleaseAssetState {
  #[serde(rename = "uploaded")]
  Uploaded,
  #[serde(rename = "draft")]
  Open,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GitHubReleaseAsset {
  pub id: u32,
  pub browser_download_url: String,
  pub name: String,
  pub state: GitHubReleaseAssetState,
  pub content_type: String,
  pub size: u64,
  pub digest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GitHubRelease {
  pub id: u32,
  pub upload_url: String,
  pub tag_name: String,
  pub target_commitish: String,
  pub name: Option<String>,
  pub body: Option<String>,
  pub assets: Vec<GitHubReleaseAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CreateGitHubReleasePayload {
  pub tag_name: String,
  pub name: Option<String>,
  pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GitHubErrorResponse {
  pub message: Option<String>,
}

pub struct GitHub {
  config: GitHubConfig,
  http: reqwest::Client,
}

impl GitHub {
  pub fn builder() -> GitHubBuilder {
    GitHubBuilder::default()
  }

  async fn create_release(
    &self,
    payload: CreateGitHubReleasePayload,
  ) -> crate::Result<GitHubRelease> {
    let resp = self
      .http
      .post(Self::api_url(format!(
        "/repos/{}/{}/releases",
        self.config.owner, self.config.repo
      )))
      .json(&payload)
      .send()
      .await?;
    Self::parse_response(resp).await
  }

  async fn get_release_by_tag_name(&self, tag_name: &str) -> crate::Result<GitHubRelease> {
    let resp = self
      .http
      .get(Self::api_url(format!(
        "/repos/{}/{}/releases/tags/{}",
        self.config.owner, self.config.repo, tag_name
      )))
      .send()
      .await?;
    Self::parse_response(resp).await
  }

  async fn upload_release_asset(
    &self,
    upload_url: &str,
    name: String,
    content_type: &str,
    data_binary: Vec<u8>,
  ) -> crate::Result<GitHubReleaseAsset> {
    let resp = self
      .http
      .post(upload_url)
      .query(&[("name", &name)])
      .header(header::CONTENT_TYPE, content_type)
      .body(data_binary)
      .send()
      .await?;
    Self::parse_response(resp).await
  }

  fn api_url(path: impl Into<String>) -> String {
    format!("https://api.github.com/{}", path.into())
  }

  async fn parse_response<T: DeserializeOwned>(resp: reqwest::Response) -> crate::Result<T> {
    match resp.status().is_success() {
      true => Ok(resp.json::<T>().await?),
      false => Err(crate::Error::GitHub {
        status: resp.status().as_u16(),
        message: resp
          .json::<GitHubErrorResponse>()
          .await?
          .message
          .unwrap_or("unknown".to_string()),
      }),
    }
  }
}

#[async_trait]
impl Remote for GitHub {
  async fn upload(&self, bundle_name: &str, version: &str, bundle: &Bundle) -> crate::Result<()> {
    let tag_name = self.config.release_tag_name(bundle_name, version);
    let name = self.config.release_name(bundle_name, version);
    let release = self
      .create_release(CreateGitHubReleasePayload {
        tag_name,
        name: Some(name),
        body: None,
      })
      .await?;
    let mut data_binary = vec![];
    BundleWriter::new(&mut data_binary).write(bundle)?;
    let asset_name = self.config.asset_name(bundle_name, version);
    self
      .upload_release_asset(&release.upload_url, asset_name, MIME_TYPE, data_binary)
      .await?;
    Ok(())
  }

  async fn download(&self, bundle_name: &str, version: &str) -> crate::Result<Bundle> {
    let tag_name = self.config.release_tag_name(bundle_name, version);
    let release = self
      .get_release_by_tag_name(&tag_name)
      .await
      .map_err(|e| match e {
        crate::Error::GitHub { status: 404, .. } => {
          crate::Error::remote_bundle_not_found("github release not found.")
        }
        _ => e,
      })?;
    let asset_name = self.config.asset_name(bundle_name, version);
    let asset = release
      .assets
      .iter()
      .find(|x| x.name == asset_name)
      .ok_or_else(|| crate::Error::remote_bundle_not_found("github release asset not found."))?;
    let bytes = self
      .http
      .get(&asset.browser_download_url)
      .send()
      .await?
      .bytes()
      .await?;
    let mut reader = Cursor::new(bytes);
    let bundle = Reader::<Bundle>::read(&mut BundleReader::new(&mut reader))?;
    Ok(bundle)
  }
}
