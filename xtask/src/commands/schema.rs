use crate::Error;
use crate::config::Config;
use biome_console::{Console, ConsoleExt, markup};
use relative_path::RelativePathBuf;
use schemars::schema_for;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn extract_schema<C>(root_dir: &Path, console: Arc<Mutex<Box<C>>>) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  let schema = schema_for!(Config);
  let schema_str = serde_json::to_string_pretty(&schema)?;

  let filepath = RelativePathBuf::from("xtask.$schema.json");
  std::fs::write(filepath.to_path(root_dir), schema_str)?;

  console.lock().unwrap().log(markup! {
    <Info>"Extract xtask config schema file: "</Info>{filepath.to_string()}
  });
  Ok(())
}
