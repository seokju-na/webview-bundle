use crate::{Builder, Config, Remote};
use async_trait::async_trait;
use reqwest::header;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use webview_bundle::{AsyncBundleWriter, Bundle, BundleWriter, Writer};

#[derive(Default, Debug, Clone)]
#[non_exhaustive]
pub struct GitHubConfig {
  pub token: Option<String>,
  pub owner: String,
  pub repo: String,
  pub api_version: Option<String>,
  pub user_agent: Option<String>,
  pub timeout: Option<u64>,
  pub pool_idle_timeout: Option<u64>,
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
}

impl Builder for GitHubBuilder {
  type Config = GitHubConfig;

  fn build(self) -> crate::Result<impl Remote> {
    if self.config.owner.is_empty() {
      return Err(crate::Error::InvalidConfig("owner is empty".to_string()));
    }
    if self.config.repo.is_empty() {
      return Err(crate::Error::InvalidConfig("repo is empty".to_string()));
    }
    let mut http = reqwest::ClientBuilder::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
      header::ACCEPT,
      header::HeaderValue::from_static("application/vnd.github+json"),
    );
    if let Some(user_agent) = self.config.user_agent.as_ref() {
      http = http.user_agent(user_agent);
    }
    if let Some(api_version) = self.config.api_version.as_ref() {
      headers.insert(
        header::HeaderName::from_static("X-GitHub-Api-Version"),
        header::HeaderValue::from_str(api_version)
          .map_err(|e| crate::Error::InvalidConfig("invalid api version".to_string()))?,
      );
    }
    if let Some(token) = self.config.token.as_ref() {
      headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(token)
          .map_err(|e| crate::Error::InvalidConfig("invalid token".to_string()))?,
      );
    }
    if let Some(timeout) = self.config.timeout {
      http = http.timeout(std::time::Duration::from_millis(timeout));
    }
    if let Some(pool_idle_timeout) = self.config.pool_idle_timeout {
      http = http.pool_idle_timeout(std::time::Duration::from_millis(pool_idle_timeout));
    }
    let http = http
      .default_headers(headers)
      .build()
      .map_err(|e| crate::Error::InvalidConfig(e.to_string()))?;
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

#[derive(Debug, Clone)]
pub struct GitHub {
  config: GitHubConfig,
  http: reqwest::Client,
}

impl GitHub {
  pub(crate) async fn list_releases(
    &self,
    per_page: Option<u8>,
    page: Option<u16>,
  ) -> crate::Result<Vec<GitHubRelease>> {
    let mut req = self.http.get(Self::api_url(format!(
      "/repos/{}/{}/releases",
      self.config.owner, self.config.repo
    )));
    if let Some(per_page) = per_page {
      req = req.query(&[("per_page", &per_page.to_string())]);
    }
    if let Some(page) = page {
      req = req.query(&[("page", &page.to_string())]);
    }
    let resp = req.send().await?;
    Self::parse_response(resp).await
  }

  pub(crate) async fn create_release(
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

  pub(crate) async fn get_release(&self, release_id: u32) -> crate::Result<GitHubRelease> {
    let resp = self
      .http
      .get(Self::api_url(format!(
        "/repos/{}/{}/releases/{}",
        self.config.owner, self.config.repo, release_id
      )))
      .send()
      .await?;
    Self::parse_response(resp).await
  }

  pub(crate) async fn get_release_by_tag_name(
    &self,
    tag_name: &str,
  ) -> crate::Result<GitHubRelease> {
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

  pub(crate) async fn upload_release_asset(
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

  pub(crate) fn api_url(path: impl Into<String>) -> String {
    format!("https://api.github.com/{}", path.into())
  }

  pub(crate) async fn parse_response<T: DeserializeOwned>(
    resp: reqwest::Response,
  ) -> crate::Result<T> {
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

  pub(crate) fn tag_name(name: &str, version: &str) -> String {
    format!("{name}_v{version}")
  }
}

#[async_trait]
impl Remote for GitHub {
  async fn upload(&self, name: &str, version: &str, bundle: &Bundle) -> crate::Result<()> {
    let release = self
      .create_release(CreateGitHubReleasePayload {
        tag_name: Self::tag_name(name, version),
        name: Some(Self::tag_name(name, version)),
        body: None,
      })
      .await?;
    let mut data_binary = vec![];
    BundleWriter::new(&mut data_binary).write(bundle)?;
    self
      .upload_release_asset(
        &release.upload_url,
        format!("{}.wvb", Self::tag_name(name, version)),
        "application/webview-bundle",
        data_binary,
      )
      .await?;
    Ok(())
  }

  async fn download(&self, name: &str, version: &str) -> crate::Result<Bundle> {
    let release = self
      .get_release_by_tag_name(&Self::tag_name(name, version))
      .await?;
    let asset = release.assets
      .iter().find()
    todo!()
  }
}
