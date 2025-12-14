use std::path::{Path, PathBuf};

pub fn normalize_path(base_dir: &Path, path: &Path) -> PathBuf {
  match path.is_absolute() {
    true => path.to_path_buf(),
    false => base_dir.join(path),
  }
}
