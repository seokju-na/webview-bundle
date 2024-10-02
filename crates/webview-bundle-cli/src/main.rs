use biome_console::{markup, ColorMode, ConsoleExt, EnvConsole};
use std::process::ExitCode;
use webview_bundle_cli::{cli_command, run, setup_panic_handler, ColorsArg};

#[tokio::main]
async fn main() -> ExitCode {
  setup_panic_handler();
  let command = cli_command().fallback_to_usage().run();

  let mut console = EnvConsole::default();
  console.set_color(to_color_mode(command.get_color()));

  match run(&mut console, command).await {
    Ok(_) => ExitCode::SUCCESS,
    Err(e) => {
      tracing::error!("{e}");
      console.error(markup! {
        <Emphasis><Error>"Error"</Error></Emphasis>": "{e.to_string()}
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
