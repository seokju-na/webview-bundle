mod commands;
mod logging;
mod options;
mod panic;
pub mod prelude;

pub use crate::commands::{cli_command, CliCommand};
pub use crate::logging::{setup_logging, LoggingKind, LoggingLevel};
pub use crate::options::{CliOptions, ColorsArg};
pub use crate::panic::setup_panic_handler;
use biome_console::{ColorMode, Console};
use prelude::*;

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn run(console: &mut dyn Console, command: CliCommand) -> Result<()> {
  match command {
    CliCommand::Pack {
      cli_options,
      dir,
      outfile,
      truncate,
    } => {
      commands::pack::pack(
        console,
        commands::pack::PackCommandData {
          cli_options,
          dir,
          outfile,
          truncate,
        },
      )
      .await
    }
    CliCommand::Extract {
      cli_options,
      outdir,
      dry_run,
      file,
    } => {
      commands::extract::extract(
        console,
        commands::extract::ExtractCommandData {
          cli_options,
          outdir,
          dry_run,
          file,
        },
      )
      .await
    }
  }
}

pub fn to_color_mode(color: Option<&ColorsArg>) -> ColorMode {
  match color {
    Some(ColorsArg::Off) => ColorMode::Disabled,
    Some(ColorsArg::Force) => ColorMode::Enabled,
    None => ColorMode::Auto,
  }
}
