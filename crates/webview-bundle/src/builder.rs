use crate::bundle::{FileDescriptorData, FileDescriptors};
use crate::{Bundle, Version};
use lz4_flex::compress_prepend_size;

#[derive(Default, Clone)]
pub struct Builder {
  version: Option<Version>,
  offset: u32,
  descriptors: FileDescriptors,
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

  pub fn add_file(mut self, path: &str, data: &[u8]) -> Self {
    let compressed = compress_prepend_size(data);
    let length = compressed.len() as u32;
    self.descriptors.insert(
      path.to_string(),
      FileDescriptorData::new(self.offset, length),
    );
    self.offset += length;
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
    let data = r#"
const a = 10;
export a;
    "#;
    let bundle = Builder::new().add_file("index.js", data.as_bytes()).build();
    assert_eq!(bundle.version(), &Version::Version1);
    assert_eq!(bundle.descriptors.len(), 1);
    assert_eq!(
      bundle.descriptors.get("index.js").unwrap(),
      &FileDescriptorData::new(1, 1)
    );
  }
}
