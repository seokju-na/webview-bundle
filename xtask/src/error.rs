use crate::actions::Actions;
use relative_path::RelativePathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Git2(#[from] git2::Error),
  #[error(transparent)]
  Semver(#[from] semver::Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  SerdeJson(#[from] serde_json::Error),
  #[error(transparent)]
  Toml(#[from] toml_edit::TomlError),
  #[error(transparent)]
  PatternError(#[from] glob::PatternError),
  #[error(transparent)]
  GlobError(#[from] glob::GlobError),
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error("github error ({status}): {message}")]
  GitHub { status: u16, message: String },
  #[error("cannot find head target")]
  CannotFindHeadTarget,
  #[error("fail to parse conventional commit")]
  ConventionalCommitParseError,
  #[error("invalid package config({name}): {message}")]
  InvalidPackageConfig { name: String, message: String },
  #[error("invalid versioned file: {path}")]
  InvalidVersionedFile { path: RelativePathBuf },
  #[error("action failed: {action:?}")]
  ActionFailed { action: Actions },
}
