use crate::logging::setup_logging;
use crate::options::CliOptions;
use crate::prelude::*;
use async_walkdir::WalkDir;
use biome_console::{markup, Console, ConsoleExt};
use futures::StreamExt;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use webview_bundle::{encode, Bundle};

#[derive(Debug)]
pub(crate) struct PackCommandData {
  pub(crate) cli_options: CliOptions,
  pub(crate) dir: OsString,
  pub(crate) outfile: Option<String>,
  pub(crate) truncate: bool,
}

pub(crate) async fn pack(console: &mut dyn Console, data: PackCommandData) -> Result<()> {
  tracing::debug!("data: {:?}", data);
  let PackCommandData {
    cli_options,
    dir,
    outfile,
    truncate,
  } = data;
  setup_logging(cli_options.log_level, cli_options.log_kind);

  let dir_path = Path::new(&dir);
  let walker = FileWalker::new(dir_path.to_path_buf()).walk().await?;
  let mut builder = Bundle::builder();
  for file in walker.files {
    console.log(markup! {
      "Target file: "<Emphasis>{file.relative_path.to_string_lossy()}</Emphasis>
    });
    builder = builder.add_file(file.relative_path, &file.data);
  }
  let bundle = builder.build();
  let mut outfile_path = outfile
    .or(
      dir_path
        .file_name()
        .map(|x| x.to_string_lossy().to_string()),
    )
    .map(|x| Path::new(&x).to_path_buf())
    .context("Invalid outfile")?;
  if outfile_path.extension().is_none() {
    outfile_path = outfile_path.with_extension("wvb");
  }
  tracing::debug!("Outfile path: {:?}", outfile_path);
  let outfile = match truncate {
    true => std::fs::File::create(&outfile_path),
    false => std::fs::File::create_new(&outfile_path),
  }
  .context("Fail to create outfile")?;
  encode(&bundle, outfile).context("Fail to encode bundle")?;
  console.log(markup! {
    "Pack webview bundle: "<Emphasis>{outfile_path.to_string_lossy()}</Emphasis>
  });
  Ok(())
}

struct File {
  relative_path: PathBuf,
  data: Vec<u8>,
}

struct FileWalker {
  base_dir: PathBuf,
  files: Vec<File>,
}

impl FileWalker {
  fn new(base_dir: PathBuf) -> Self {
    Self {
      base_dir,
      files: vec![],
    }
  }

  async fn walk(mut self) -> Result<Self> {
    let mut entries = WalkDir::new(&self.base_dir);
    loop {
      match entries.next().await {
        Some(Ok(entry)) => {
          tracing::debug!("Walk entry: {:?}", entry);
          let meta = entry.metadata().await.context("Fail to read metadata")?;
          if meta.is_symlink() {
            tracing::warn!(
              "Path is symbolic link. Skipping: {}",
              entry.path().display()
            );
            continue;
          }
          if !meta.is_file() {
            continue;
          }
          let relative_path =
            pathdiff::diff_paths(entry.path(), &self.base_dir).context("Path is not relative")?;
          let data = tokio::fs::read(entry.path())
            .await
            .context("Error while reading file")?;
          tracing::debug!("Add path to file: {}", entry.path().display());
          self.files.push(File {
            relative_path,
            data,
          })
        }
        Some(Err(e)) => {
          tracing::error!("Error while reading path: {e}");
        }
        None => break,
      }
    }
    Ok(self)
  }
}
