mod util;

use anyhow::anyhow;
use biome_console::{markup, BufferConsole, Console, ConsoleExt};
use webview_bundle_cli::prelude::*;
use webview_bundle_cli::{cli_command, run};

async fn test_run<'a>(console: &'a mut dyn Console, args: bpaf::Args<'a>) -> Result<()> {
  let command = cli_command().run_inner(args);
  match command {
    Ok(x) => {
      run(console, x).await?;
      Ok(())
    }
    Err(failure) => match failure {
      bpaf::ParseFailure::Stdout(doc, _) => {
        console.log(markup! {{doc.to_string()}});
        Ok(())
      }
      bpaf::ParseFailure::Completion(message) => Err(anyhow!(message)),
      bpaf::ParseFailure::Stderr(doc) => Err(anyhow!(doc)),
    },
  }
}

#[tokio::test]
async fn unknown_command() {
  let mut console = BufferConsole::default();
  let result = test_run(
    &mut console,
    bpaf::Args::from(["unknown", "--help"].as_slice()),
  );
  assert!(result.await.is_ok());
}

const JS_SAMPLE1: &str = r#"export const A = 10;

export function hello() {
  return 20;
}
"#;

#[tokio::test]
async fn pack_and_extract() {
  let mut console = BufferConsole::default();
  let xfs = util::Xfs::new().await;
  let mut dir = xfs.base_dir_path_buf();
  dir.push("bundle");

  xfs
    .write_file("bundle/index.js", JS_SAMPLE1.as_bytes())
    .await;

  let mut outfile = xfs.base_dir_path_buf();
  outfile.push("bundle.wvb");

  let result = test_run(
    &mut console,
    bpaf::Args::from(
      [
        "pack",
        dir.to_str().unwrap(),
        "-o",
        outfile.to_str().unwrap(),
      ]
      .as_slice(),
    ),
  )
  .await;
  assert!(result.is_ok(), "pack result: {result:?}");

  let mut outdir = xfs.base_dir_path_buf();
  outdir.push("output");

  let result = test_run(
    &mut console,
    bpaf::Args::from(
      [
        "extract",
        outfile.to_str().unwrap(),
        "-o",
        outdir.to_str().unwrap(),
      ]
      .as_slice(),
    ),
  )
  .await;
  assert!(result.is_ok(), "extract result: {result:?}");
  assert_eq!(
    xfs.read_file("output/index.js").await,
    JS_SAMPLE1.as_bytes()
  );
}
