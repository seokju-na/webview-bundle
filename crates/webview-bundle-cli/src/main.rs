use crate::commands::{cli_command, CliCommand};
use crate::options::ColorsArg;
use crate::panic::setup_panic_handler;
use biome_console::{markup, ColorMode, ConsoleExt, EnvConsole};
use std::process::ExitCode;

mod commands;
mod logging;
mod options;
mod panic;
mod prelude;

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
pub async fn main() -> ExitCode {
  setup_panic_handler();
  let command = cli_command().fallback_to_usage().run();

  let mut console = EnvConsole::default();
  console.set_color(to_color_mode(command.get_color()));

  let result = match command {
    CliCommand::Pack {
      cli_options,
      dir,
      outfile,
      truncate,
    } => {
      commands::pack::pack(
        &mut console,
        commands::pack::PackCommandData {
          cli_options,
          dir,
          outfile,
          truncate,
        },
      )
      .await
    }
  };
  match result {
    Ok(_) => ExitCode::SUCCESS,
    Err(e) => {
      tracing::error!("{e}");
      console.error(markup! {
        <Error>{e.to_string()}</Error>
      });
      ExitCode::FAILURE
    }
  }
}

fn to_color_mode(color: Option<&ColorsArg>) -> ColorMode {
  match color {
    Some(ColorsArg::Off) => ColorMode::Disabled,
    Some(ColorsArg::Force) => ColorMode::Enabled,
    None => ColorMode::Auto,
  }
}
