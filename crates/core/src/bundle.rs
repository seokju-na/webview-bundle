use crate::builder::Builder;
use crate::index::Index;
use crate::version::Version;
use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::{Decode, Encode};
use lz4_flex::decompress_size_prepended;
use std::io::{Cursor, Read};
use std::ops::{Deref, DerefMut};

// 🌐🎁
pub(crate) const MAGIC_BYTES_LEN: usize = 8;
pub const HEADER_MAGIC_BYTES: [u8; MAGIC_BYTES_LEN] =
  [0xf0, 0x9f, 0x8c, 0x90, 0xf0, 0x9f, 0x8e, 0x81];
pub(crate) const VERSION_BYTES_LEN: usize = 1;
pub(crate) const INDEX_SIZE_BYTES_LEN: usize = 4;
pub(crate) const FILE_DESCRIPTORS_SIZE_BYTES_LENGTH: usize = 4;

#[derive(Debug, PartialEq, Clone)]
pub struct Bundle {
  pub(crate) version: Version,
  pub(crate) index: Index,
  pub(crate) data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BundleFile {
  pub path: String,
  pub data: Vec<u8>,
}

impl Bundle {
  pub fn version(&self) -> &Version {
    &self.version
  }

  pub fn index(&self) -> &Index {
    &self.index
  }

  pub fn response<S: Into<String>>(&self, path: S) -> Option<http::Response<&[u8]>> {
    todo!()
  }

  pub fn read_all_files(&self) -> crate::Result<Vec<BundleFile>> {
    let mut files = vec![];
    for path in self.descriptors.keys() {
      let file = self.read_file(path)?;
      files.push(file);
    }
    Ok(files)
  }

  pub fn read_file(&self, path: &str) -> crate::Result<BundleFile> {
    let &FileDescriptorData { offset, length } = self
      .descriptors
      .get(path)
      .ok_or(crate::Error::FileNotFound)?;
    let mut cursor = Cursor::new(&self.data);
    cursor.set_position(offset.into());
    let mut buf = vec![0; length as usize];
    cursor.read_exact(&mut buf)?;
    let data = decompress_size_prepended(&buf)?;
    let file = BundleFile {
      path: String::from(path),
      data,
    };
    Ok(file)
  }

  pub fn read_file_data(&self, path: &str) -> crate::Result<Vec<u8>> {
    let file = self.read_file(path)?;
    Ok(file.data)
  }

  pub fn builder() -> Builder {
    Builder::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_file() {
    let data = r#"
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
    let bundle = Bundle::builder()
      .add_file("index.jsx", data.as_bytes())
      .build();
    assert_eq!(bundle.read_file_data("index.jsx").unwrap(), data.as_bytes());
  }

  #[test]
  fn read_all_files() {
    let data1 = r#"<h1>Hello World</h1>"#;
    let data2 = r#"const a = 10;"#;
    let bundle = Bundle::builder()
      .add_file("index.html", data1.as_bytes())
      .add_file("index.js", data2.as_bytes())
      .build();
    let files = bundle.read_all_files().unwrap();
    assert_eq!(files.len(), 2);
  }

  #[test]
  fn read_file_err() {
    let data1 = r#"<h1>Hello World</h1>"#;
    let data2 = r#"const a = 10;"#;
    let bundle = Bundle::builder()
      .add_file("index.html", data1.as_bytes())
      .add_file("index.js", data2.as_bytes())
      .build();
    assert!(bundle.read_file_data("index.html").is_ok());
    assert!(bundle.read_file_data("index.js").is_ok());
    assert!(matches!(
      bundle.read_file_data("not_exists.js").unwrap_err(),
      crate::Error::FileNotFound,
    ));
  }
}
