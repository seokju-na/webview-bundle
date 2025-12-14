use std::path::PathBuf;

fn fixtures_dir() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("fixtures")
}

pub struct Fixtures {
  base_dir: PathBuf,
}

impl Fixtures {
  pub fn bundles() -> Self {
    Self {
      base_dir: fixtures_dir().join("bundles"),
    }
  }

  pub fn get_path(&self, path: &str) -> PathBuf {
    self.base_dir.join(path)
  }
}
