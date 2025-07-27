use rand::{distributions, thread_rng, Rng};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Xfs {
  base_dir: PathBuf,
}

impl Xfs {
  pub async fn new() -> Self {
    let mut dir = std::env::temp_dir();
    dir.push(Self::rand(21));

    tokio::fs::DirBuilder::new()
      .recursive(true)
      .create(&dir)
      .await
      .expect("Fail to create xfs directory");
    Self { base_dir: dir }
  }

  pub fn base_dir_path(&self) -> &Path {
    Path::new(&self.base_dir)
  }

  pub fn base_dir_path_buf(&self) -> PathBuf {
    self.base_dir_path().to_path_buf()
  }

  pub async fn ensure_dir(&self, path: impl AsRef<Path>) {
    let mut dir_path = self.base_dir_path_buf();
    dir_path.push(path);
    tokio::fs::DirBuilder::new()
      .recursive(true)
      .create(&dir_path)
      .await
      .expect("Fail to create directory");
  }

  pub async fn write_file(&self, path: impl AsRef<Path>, data: impl AsRef<[u8]>) {
    if let Some(parent_path) = path.as_ref().parent() {
      self.ensure_dir(parent_path).await;
    }

    let mut file_path = self.base_dir_path_buf();
    file_path.push(path);

    tokio::fs::write(file_path, data)
      .await
      .expect("Fail to write file");
  }

  pub async fn read_file(&self, path: impl AsRef<Path>) -> Vec<u8> {
    let mut file_path = self.base_dir_path_buf();
    file_path.push(path);

    let data = tokio::fs::read(file_path).await.expect("Fail to read file");
    data
  }

  fn rand(size: usize) -> String {
    thread_rng()
      .sample_iter(&distributions::Alphanumeric)
      .take(size)
      .map(char::from)
      .collect()
  }
}

impl Drop for Xfs {
  fn drop(&mut self) {
    std::fs::remove_dir_all(&self.base_dir).expect("Fail to remove temp directory");
  }
}
