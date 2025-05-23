use crate::builder::Builder;
use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use lz4_flex::decompress_size_prepended;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read};
use std::ops::{Deref, DerefMut};

// 🌐🎁
pub const HEADER_MAGIC_BYTES: [u8; 8] = [0xf0, 0x9f, 0x8c, 0x90, 0xf0, 0x9f, 0x8e, 0x81];
pub(crate) const VERSION_BYTES_LENGTH: usize = 1;
pub(crate) const FILE_DESCRIPTORS_SIZE_BYTES_LENGTH: usize = 4;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
  pub const fn bytes(&self) -> [u8; 1] {
    match self {
      Version::Version1 => [0x01],
    }
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::Version1 => "v1",
    };
    f.write_str(s)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FileDescriptorData {
  pub offset: u32,
  pub length: u32,
}

impl FileDescriptorData {
  pub fn new(offset: u32, length: u32) -> Self {
    Self { offset, length }
  }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileDescriptors(pub(crate) HashMap<String, FileDescriptorData>);

impl Encode for FileDescriptors {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
    let data = self
      .0
      .iter()
      .map(|(path, data)| (path, (data.offset, data.length)))
      .collect::<HashMap<_, _>>();
    Encode::encode(&data, encoder)?;
    Ok(())
  }
}

impl<T> Decode<T> for FileDescriptors {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
    let data = HashMap::<String, (u32, u32)>::decode(decoder)?
      .into_iter()
      .map(|(path, (offset, length))| (path, FileDescriptorData { offset, length }))
      .collect::<HashMap<_, _>>();
    Ok(Self(data))
  }
}

impl Deref for FileDescriptors {
  type Target = HashMap<String, FileDescriptorData>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for FileDescriptors {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl FileDescriptors {
  pub fn get(&self, path: &str) -> Option<&FileDescriptorData> {
    self.deref().get(path)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bundle {
  pub(crate) version: Version,
  pub(crate) descriptors: FileDescriptors,
  pub(crate) data: Vec<u8>,
}

impl Bundle {
  pub fn version(&self) -> &Version {
    &self.version
  }

  pub fn descriptors(&self) -> &FileDescriptors {
    &self.descriptors
  }

  pub fn read_file(&self, path: &str) -> crate::Result<Vec<u8>> {
    let &FileDescriptorData { offset, length } = self
      .descriptors
      .get(path)
      .ok_or(crate::Error::FileNotFound)?;
    let mut cursor = Cursor::new(&self.data);
    cursor.set_position(offset.into());
    let mut buf = vec![0; length as usize];
    cursor.read_exact(&mut buf)?;
    let file = decompress_size_prepended(&buf)?;
    Ok(file)
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
    let bundle = Bundle::builder()
      .add_file("index.jsx", file.as_bytes())
      .build();
    assert_eq!(bundle.read_file("index.jsx").unwrap(), file.as_bytes());
  }

  #[test]
  fn read_file_err() {
    let file1 = r#"<h1>Hello World</h1>"#;
    let file2 = r#"const a = 10;"#;
    let bundle = Bundle::builder()
      .add_file("index.html", file1.as_bytes())
      .add_file("index.js", file2.as_bytes())
      .build();
    assert!(bundle.read_file("index.html").is_ok());
    assert!(bundle.read_file("index.js").is_ok());
    assert!(matches!(
      bundle.read_file("not_exists.js").unwrap_err(),
      crate::Error::FileNotFound,
    ));
  }
}
