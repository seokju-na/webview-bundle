use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptConfig {
  pub command: String,
  pub args: Option<Vec<String>>,
  pub cwd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
  pub versioned_files: Vec<String>,
  pub changelog: Option<String>,
  pub scopes: Vec<String>,
  pub before_publish_scripts: Option<Vec<ScriptConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRepoConfig {
  pub owner: String,
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubConfig {
  pub repo: GitHubRepoConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactFileConfig {
  pub source: String,
  pub dist: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactsConfig {
  pub dir: String,
  pub files: Vec<ArtifactFileConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub root_changelog: Option<String>,
  pub packages: HashMap<String, PackageConfig>,
  pub github: GitHubConfig,
  pub artifacts: ArtifactsConfig,
}

impl Config {
  pub fn parse(content: String) -> Result<Self, crate::Error> {
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
  }

  pub fn load(root_dir: &Path) -> Result<Self, crate::Error> {
    let content = std::fs::read_to_string(root_dir.join("xtask.json"))?;
    Self::parse(content)
  }
}
