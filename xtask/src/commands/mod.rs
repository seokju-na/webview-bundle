mod release;

use crate::cli::{CliOptions, ColorsArg, cli_options};
use bpaf::Bpaf;

pub use release::release;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum CliCommand {
  #[bpaf(command)]
  Release {
    #[bpaf(external(cli_options), hide_usage)]
    cli_options: CliOptions,

    /// Perform all checks without releasing.
    #[bpaf(long("dry-run"), switch)]
    dry_run: bool,
  },
}

impl CliCommand {
  const fn cli_options(&self) -> Option<&CliOptions> {
    match self {
      CliCommand::Release { cli_options, .. } => Some(cli_options),
    }
  }

  pub const fn get_color(&self) -> Option<&ColorsArg> {
    match self.cli_options() {
      Some(cli_options) => cli_options.colors.as_ref(),
      None => None,
    }
  }
}
