use std::path::PathBuf;

pub struct Fixtures {
  base_dir: PathBuf,
}

impl Fixtures {
  pub fn new() -> Self {
    Self {
      base_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures"),
    }
  }

  pub fn get_path(&self, path: &str) -> PathBuf {
    self.base_dir.join(path)
  }
}
