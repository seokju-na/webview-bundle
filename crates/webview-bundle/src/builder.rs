use crate::bundle::FileDescriptor;
use crate::{Bundle, Version};
use lz4_flex::compress_prepend_size;
use std::path::Path;

#[derive(Default, Clone)]
pub struct Builder {
  version: Option<Version>,
  offset: u64,
  descriptors: Vec<FileDescriptor>,
  data: Vec<u8>,
}

impl Builder {
  pub(crate) fn new() -> Self {
    Default::default()
  }

  pub fn version(mut self, version: Version) -> Self {
    self.version = Some(version);
    self
  }

  pub fn add_file<P: AsRef<Path>>(mut self, path: P, data: &[u8]) -> Self {
    let compressed = compress_prepend_size(data);
    let length = compressed.len() as u64;
    let descriptor = FileDescriptor {
      path: path.as_ref().to_string_lossy().to_string(),
      offset: self.offset,
      length,
    };
    self.offset += length;
    self.descriptors.push(descriptor);
    self.data.extend_from_slice(&compressed);
    self
  }

  pub fn build(self) -> Bundle {
    let version = self.version.unwrap_or_default();
    Bundle {
      version,
      descriptors: self.descriptors,
      data: self.data,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn build() {
    let path = Path::new("index.js");
    let data = r#"
const a = 10;
export a;
    "#;
    let bundle = Builder::new().add_file(path, data.as_bytes()).build();
    assert_eq!(bundle.version(), &Version::Version1);
    assert_eq!(bundle.descriptors.len(), 1);
    assert_eq!(bundle.descriptors.first().unwrap().path, "index.js");
  }
}
