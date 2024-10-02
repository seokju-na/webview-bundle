mod bundle;
pub mod extract;
pub mod pack;

use crate::options::{cli_options, CliOptions, ColorsArg};
use crate::VERSION;
use bpaf::Bpaf;
use std::ffi::OsString;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version(VERSION))]
pub enum CliCommand {
  /// Create webview bundle archive
  #[bpaf(command)]
  Pack {
    #[bpaf(external(cli_options), hide_usage)]
    cli_options: CliOptions,

    /// Outfile path to create webview bundle archive.
    /// If not provided, will create file with directory name.
    /// If extension not set, will automatically add extension (`.wvb`)
    #[bpaf(short('o'), long("outfile"), argument("PATH"), optional)]
    outfile: Option<String>,

    /// Options to truncate outfile if file already exists.
    #[bpaf(long("truncate"), switch)]
    truncate: bool,

    /// Directory path to archive as webview bundle format.
    #[bpaf(positional("PATH"))]
    dir: OsString,
  },

  /// Extract webview bundle archive
  #[bpaf(command)]
  Extract {
    #[bpaf(external(cli_options), hide_usage)]
    cli_options: CliOptions,

    /// Don't create extract files on disk, instead just look what inside on the webview bundle file.
    #[bpaf(long("dry-run"), switch)]
    dry_run: bool,

    /// Outdir path to extract webview bundle files.
    /// If not provided, will use webview bundle file name as directory.
    #[bpaf(short('o'), long("outdir"), argument("PATH"), optional)]
    outdir: Option<String>,

    /// Webview bundle file path to extract.
    #[bpaf(positional("PATH"))]
    file: OsString,
  },
}

impl CliCommand {
  const fn cli_options(&self) -> Option<&CliOptions> {
    match self {
      CliCommand::Pack { cli_options, .. } | CliCommand::Extract { cli_options, .. } => {
        Some(cli_options)
      }
    }
  }

  pub const fn get_color(&self) -> Option<&ColorsArg> {
    match self.cli_options() {
      Some(cli_options) => cli_options.colors.as_ref(),
      None => None,
    }
  }
}
