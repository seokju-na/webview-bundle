use crate::Error;
use reqwest::header;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub struct GitHubClient {
  client: reqwest::blocking::Client,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GitHubErrorData {
  pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubCreateReleasePayload {
  pub tag_name: String,
  pub name: Option<String>,
  pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubRelease {
  pub html_url: String,
  pub id: i32,
  pub tag_name: String,
}

impl GitHubClient {
  pub fn new(token: &Option<String>) -> Result<Self, Error> {
    let mut headers = header::HeaderMap::new();
    if let Some(token) = token {
      headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(token).unwrap(),
      );
    }
    headers.insert(
      header::ACCEPT,
      header::HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
      "X-GitHub-Api-Version",
      header::HeaderValue::from_static("2022-11-28"),
    );

    let client = reqwest::blocking::Client::builder()
      .default_headers(headers)
      .user_agent("webview-bundle")
      .build()?;
    Ok(Self { client })
  }

  pub fn create_release(
    &self,
    owner: &str,
    repo: &str,
    payload: &GitHubCreateReleasePayload,
  ) -> Result<GitHubRelease, Error> {
    let resp = self
      .client
      .post(format!(
        "https://api.github.com/repos/{}/{}/releases",
        owner, repo
      ))
      .json(payload)
      .send()?;
    let result = Self::parse_response::<GitHubRelease>(resp)?;
    Ok(result)
  }

  fn parse_response<T: DeserializeOwned>(resp: reqwest::blocking::Response) -> Result<T, Error> {
    let status = resp.status();
    if !status.is_success() {
      let json = resp.json::<GitHubErrorData>().unwrap_or_default();
      return Err(Error::GitHub {
        status: status.as_u16(),
        message: json.message.unwrap_or_default(),
      });
    }
    let json = resp.json::<T>()?;
    Ok(json)
  }
}
