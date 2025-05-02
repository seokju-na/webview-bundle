mod actions;
mod cargo;
mod changelog;
mod changes;
mod cli;
mod commands;
mod config;
mod conventional_commit;
mod error;
mod exec;
mod package;
mod version;
mod versioned_file;
mod versioned_git_tag;

pub use error::Error;
use std::path::Path;

use crate::cli::ColorsArg;
use crate::commands::{CliCommand, cli_command};
use biome_console::{ColorMode, ConsoleExt, EnvConsole, markup};
use project_root::get_project_root;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> ExitCode {
  let root_dir_buf = get_project_root().expect("Unable to find project root");
  let root_dir = Path::new(&root_dir_buf);

  let command = cli_command().fallback_to_usage().run();

  let mut console = EnvConsole::default();
  console.set_color(to_color_mode(command.get_color()));

  let cons = Arc::new(Mutex::new(Box::new(console)));

  let result = match command {
    CliCommand::Release {
      cli_options: _,
      prerelease,
      dry_run,
    } => commands::release(root_dir, cons, prerelease, dry_run),
  };
  match result {
    Ok(_) => ExitCode::SUCCESS,
    Err(e) => {
      let mut console = EnvConsole::default();
      console.error(markup! {
        <Error>"Error: "</Error>{format!("{:?}", e)}
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
