use crate::builder::Builder;
use bincode::{Decode, Encode};
use lz4_flex::decompress_size_prepended;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read};
use std::path::Path;

// 🌐🎁
pub const HEADER_MAGIC_BYTES: [u8; 8] = [0xf0, 0x9f, 0x8c, 0x90, 0xf0, 0x9f, 0x8e, 0x81];
pub(crate) const VERSION_BYTES_LENGTH: usize = 4;
pub(crate) const FILE_DESCRIPTORS_SIZE_BYTES_LENGTH: usize = 4;

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
  /// Version 1
  Version1,
}

impl Default for Version {
  fn default() -> Self {
    Self::Version1
  }
}

impl Version {
  pub fn bytes(&self) -> &[u8; 4] {
    match self {
      Version::Version1 => &[0x76, 0x31, 0, 0],
    }
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::Version1 => "version1",
    };
    f.write_str(s)
  }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct FileDescriptor {
  pub(crate) path: String,
  pub(crate) offset: u64,
  pub(crate) length: u64,
}

impl FileDescriptor {
  pub(crate) fn path_matches<P: AsRef<Path>>(&self, path: &P) -> bool {
    self.path == path.as_ref().to_string_lossy()
  }

  pub fn path(&self) -> &String {
    &self.path
  }

  pub fn size(&self) -> u64 {
    self.length
  }
}

#[derive(Debug, PartialEq)]
pub struct Bundle {
  pub(crate) version: Version,
  pub(crate) descriptors: Vec<FileDescriptor>,
  pub(crate) data: Vec<u8>,
}

impl Bundle {
  pub fn version(&self) -> &Version {
    &self.version
  }

  pub fn descriptors(&self) -> &[FileDescriptor] {
    &self.descriptors
  }

  pub fn read_file<P: AsRef<Path>>(&self, path: P) -> crate::Result<Vec<u8>> {
    let descriptor = self
      .find_descriptor(path)
      .ok_or(crate::Error::FileNotFound)?;
    let mut cursor = Cursor::new(&self.data);
    cursor.set_position(descriptor.offset);
    let mut buf = vec![0; descriptor.length as usize];
    cursor.read_exact(&mut buf)?;
    let file = decompress_size_prepended(&buf)?;
    Ok(file)
  }

  pub fn builder() -> Builder {
    Builder::new()
  }

  fn find_descriptor<P: AsRef<Path>>(&self, path: P) -> Option<&FileDescriptor> {
    self.descriptors.iter().find(|x| x.path_matches(&path))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_file() {
    let path = Path::new("index.jsx");
    let file = r#"
import React, { useState } from 'react';

export function MyComponent() {
  const [count, setCount] = useState(0);
  return (
    <div>
      <h1>Count: {count}</h1>
      <button onClick={() => setCount(x => x + 1)}>increse</button>
    </div>
  );
}
    "#;
    let bundle = Bundle::builder().add_file(path, file.as_bytes()).build();
    assert_eq!(bundle.read_file(path).unwrap(), file.as_bytes());
  }

  #[test]
  fn read_file_err() {
    let path1 = Path::new("index.html");
    let file1 = r#"<h1>Hello World</h1>"#;
    let path2 = Path::new("index.js");
    let file2 = r#"const a = 10;"#;
    let bundle = Bundle::builder()
      .add_file(path1, file1.as_bytes())
      .add_file(path2, file2.as_bytes())
      .build();
    assert!(bundle.read_file(path1).is_ok());
    assert!(bundle.read_file(path2).is_ok());
    assert!(matches!(
      bundle.read_file(Path::new("other.js")).unwrap_err(),
      crate::Error::FileNotFound,
    ));
  }
}
