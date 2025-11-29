use std::path::{Path, PathBuf};

pub struct Tmp {
  base_dir: PathBuf,
}

impl Tmp {
  pub fn new() -> Self {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tmp");
    todo!()
  }

  pub fn new_with_base_dir(base_dir: &Path) -> Self {
    let base_dir = base_dir.to_path_buf();
    std::fs::create_dir_all(&base_dir).expect("fail to create tmp dir");
    Self { base_dir }
  }

  pub fn base_dir(&self) -> &Path {
    &self.base_dir
  }
}
