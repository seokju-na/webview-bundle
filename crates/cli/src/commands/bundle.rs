use anyhow::Context;
use std::path::Path;
use tokio::io::AsyncReadExt;
use webview_bundle::{decode, Bundle};

pub(crate) async fn read(file_path: impl AsRef<Path>) -> anyhow::Result<Bundle> {
  let mut file = tokio::fs::File::open(file_path)
    .await
    .context("File not found")?;
  let mut data = vec![];
  file
    .read_to_end(&mut data)
    .await
    .context("Fail to read file data")?;
  let bundle = decode(data)?;
  Ok(bundle)
}
