#![cfg(any(target_os = "macos", target_os = "linux", windows))]

use biome_console::EnvConsole;
use napi::bindgen_prelude::*;
use webview_bundle_cli::{cli_command, run as run_command, setup_panic_handler, to_color_mode};

#[napi_derive::napi]
pub async fn run(args: Vec<String>) -> Result<()> {
  setup_panic_handler();
  let command = match cli_command().fallback_to_usage().run_inner(args.as_slice()) {
    Ok(c) => c,
    Err(e) => {
      e.print_message(100);
      if e.exit_code() == 0 {
        return Ok(());
      }
      return Err(Error::new(Status::InvalidArg, "Invalid arguments"));
    }
  };
  let mut console = EnvConsole::default();
  console.set_color(to_color_mode(command.get_color()));
  match run_command(&mut console, command).await {
    Ok(_) => Ok(()),
    Err(e) => Err(Error::new(Status::GenericFailure, format!("{:#}", e))),
  }
}
