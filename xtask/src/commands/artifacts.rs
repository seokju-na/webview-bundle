use crate::Error;
use crate::config::Config;
use biome_console::{Console, ConsoleExt, markup};
use glob::glob;
use relative_path::{PathExt, RelativePathBuf};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn spread_artifacts<C>(root_dir: &Path, console: Arc<Mutex<Box<C>>>) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  let mut cons = console.lock().unwrap();
  let config = Config::load(root_dir)?;
  let dir = RelativePathBuf::from(&config.artifacts.dir);
  let pattern = dir.to_path(root_dir).join("**/*");
  let files = glob(pattern.to_str().unwrap())?
    .filter_map(Result::ok)
    .filter(|x| x.is_file())
    .collect::<Vec<_>>();
  if files.is_empty() {
    cons.log(markup! {
      <Warn>"No files found"</Warn>
    });
    return Ok(());
  }
  let len = files.len();
  cons.log(markup! {
    "Found "<Info>{len}</Info>" file(s) to spread"
  });
  for file in files {
    let filepath = file
      .relative_to(RelativePathBuf::from(&config.artifacts.dir).to_path(root_dir))
      .unwrap();
    let artifact_file = config.artifacts.files.iter().find(|x| {
      x.source
        .parse::<glob::Pattern>()
        .map(|pattern| pattern.matches(filepath.as_ref()))
        .ok()
        .unwrap_or_default()
    });
    match artifact_file {
      Some(artifact_file) => {
        let src = file.clone();
        let dest = RelativePathBuf::from(&artifact_file.dist)
          .to_path(root_dir)
          .join(src.file_name().unwrap());
        let _ = std::fs::create_dir_all(dest.parent().unwrap());
        std::fs::copy(&src, &dest)?;
        cons.log(markup! {
          <Success>"File copied: "</Success>{filepath.to_string()}
        });
      }
      None => {
        cons.log(markup! {
          <Warn>"Skip because no artifact file found: "</Warn>{filepath.to_string()}
        });
      }
    }
  }
  Ok(())
}

pub fn merge_artifacts<C>(root_dir: &Path, console: Arc<Mutex<Box<C>>>) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  let mut cons = console.lock().unwrap();
  let config = Config::load(root_dir)?;
  let mut files = vec![];
  for artifact_file in &config.artifacts.files {
    let pattern = RelativePathBuf::from(&artifact_file.source).to_path(root_dir);
    let found_files = glob(pattern.to_str().unwrap())?
      .filter_map(Result::ok)
      .filter(|x| x.is_file())
      .collect::<Vec<_>>();
    files.extend(found_files);
  }
  if files.is_empty() {
    cons.log(markup! {
      <Warn>"No files found"</Warn>
    });
    return Ok(());
  }
  let len = files.len();
  cons.log(markup! {
    "Found "<Info>{len}</Info>" file(s) to merge"
  });
  for file in files {
    let filepath = file.relative_to(root_dir).unwrap();
    let src = file.clone();
    let dest = RelativePathBuf::from(&config.artifacts.dir)
      .to_path(root_dir)
      .join(filepath.to_string());
    let _ = std::fs::create_dir_all(dest.parent().unwrap());
    std::fs::copy(&src, &dest)?;
    cons.log(markup! {
      <Success>"File copied: "</Success>{filepath.to_string()}
    });
  }
  Ok(())
}
