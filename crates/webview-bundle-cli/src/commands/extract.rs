use crate::commands::bundle;
use crate::logging::setup_logging;
use crate::options::CliOptions;
use crate::prelude::*;
use anyhow::anyhow;
use biome_console::{markup, Console, ConsoleExt};
use human_bytes::human_bytes;
use std::ffi::OsString;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use webview_bundle::Bundle;

#[derive(Debug)]
pub(crate) struct ExtractCommandData {
  pub(crate) cli_options: CliOptions,
  pub(crate) file: OsString,
  pub(crate) outdir: Option<String>,
  pub(crate) dry_run: bool,
}

pub(crate) async fn extract(console: &mut dyn Console, data: ExtractCommandData) -> Result<()> {
  tracing::debug!("data: {:?}", data);
  let ExtractCommandData {
    cli_options,
    file,
    outdir,
    dry_run,
  } = data;
  setup_logging(cli_options.log_level, cli_options.log_kind);

  let file_path = Path::new(&file);
  let bundle = bundle::read(file_path).await?;

  console.log(markup! {
    "Webview bundle info: `"{file_path.to_string_lossy()}"`\n"
    "  "<Info>"Version:"</Info>" "{bundle.version().to_string()}"\n"
    "  "<Info>"Files:"</Info>
  });
  for (path, data) in bundle.descriptors().iter() {
    let file_size = human_bytes(data.length as f64);
    console.log(markup! {
      "    "{path}" "<Dim>"("{file_size}")"</Dim>
    });
  }

  if dry_run {
    tracing::debug!("Skip for write files on disk, because it's dry run.");
    return Ok(());
  }

  let outdir = outdir
    .or(
      file_path
        .file_stem()
        .map(|x| x.to_string_lossy().to_string()),
    )
    .map(|x| Path::new(&x).to_path_buf())
    .context("Invalid outdir")?;

  if tokio::fs::metadata(&outdir).await.is_ok() {
    return Err(anyhow!("Outdir already exists."));
  }

  for path in bundle.descriptors().keys() {
    extract_file(&outdir, &bundle, path).await?;
  }

  console.log(markup! {
    <Success>"Extract completed"</Success>": "{outdir.to_string_lossy()}
  });
  Ok(())
}

async fn extract_file(outdir: &Path, bundle: &Bundle, path: &String) -> Result<()> {
  let mut file_path = outdir.to_path_buf();
  file_path.push(path);

  if let Some(dir) = file_path.parent() {
    // ensure directory
    tokio::fs::DirBuilder::new()
      .recursive(true)
      .create(dir)
      .await
      .context("Fail to create directory")?;
  }

  // write file
  let data = bundle.read_file(path)?;
  let mut file = tokio::fs::File::create_new(&file_path)
    .await
    .context("Fail to create file")?;
  file.write_all(&data).await.context("Fail to write file")?;
  Ok(())
}
