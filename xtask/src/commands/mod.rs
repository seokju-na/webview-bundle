mod artifacts;
mod release;

use crate::cli::{CliOptions, ColorsArg, cli_options};
use bpaf::Bpaf;
use std::str::FromStr;

pub use artifacts::{merge_artifacts, spread_artifacts};
pub use release::release;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum CliCommand {
  #[bpaf(command)]
  Release {
    #[bpaf(external(cli_options), hide_usage)]
    cli_options: CliOptions,

    #[bpaf(long("prerelease"))]
    prerelease: Option<PrereleaseOptions>,

    #[bpaf(long("github-token"))]
    github_token: Option<String>,

    /// Perform all checks without releasing.
    #[bpaf(long("dry-run"), switch)]
    dry_run: bool,
  },
  #[bpaf(command)]
  SpreadArtifacts {
    #[bpaf(external(cli_options), hide_usage)]
    cli_options: CliOptions,
  },
  #[bpaf(command)]
  MergeArtifacts {
    #[bpaf(external(cli_options), hide_usage)]
    cli_options: CliOptions,
  },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrereleaseOptions {
  pub id: String,
  pub num: u8,
}

impl FromStr for PrereleaseOptions {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.splitn(2, '.');
    let id = parts.next().ok_or("prerelease id is missing")?.to_string();
    let num = parts
      .next()
      .ok_or("prerelease number is missing")?
      .parse::<u8>()
      .map_err(|_| "prerelease number is not a number")?;
    Ok(Self { id, num })
  }
}

impl CliCommand {
  const fn cli_options(&self) -> Option<&CliOptions> {
    match self {
      CliCommand::Release { cli_options, .. }
      | CliCommand::SpreadArtifacts { cli_options, .. }
      | CliCommand::MergeArtifacts { cli_options, .. } => Some(cli_options),
    }
  }

  pub const fn get_color(&self) -> Option<&ColorsArg> {
    match self.cli_options() {
      Some(cli_options) => cli_options.colors.as_ref(),
      None => None,
    }
  }
}
