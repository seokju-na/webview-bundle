use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::io::Read;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Clone)]
pub struct IndexEntry {
  offset: u32,
  len: u32,
  pub headers: HeaderMap,
}

impl IndexEntry {
  pub fn new(offset: u32, len: u32) -> Self {
    Self {
      offset,
      len,
      headers: HeaderMap::default(),
    }
  }

  pub fn offset(&self) -> u32 {
    self.offset
  }

  pub fn len(&self) -> u32 {
    self.len
  }
}

impl Encode for IndexEntry {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
    let mut pairs: Vec<(String, Vec<u8>)> = Vec::with_capacity(self.headers.len());
    for (name, value) in self.headers.iter() {
      pairs.push((name.as_str().to_string(), value.as_bytes().to_vec()));
    }
    let tuple = (self.offset, self.len, pairs);
    tuple.encode(encoder)?;
    Ok(())
  }
}

impl<T> Decode<T> for IndexEntry {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
    let (offset, len, pairs): (u32, u32, Vec<(String, Vec<u8>)>) = Decode::decode(decoder)?;
    let mut headers = HeaderMap::new();
    for (name, value_bytes) in pairs {
      let header_name = HeaderName::try_from(name.as_str())
        .map_err(|_| DecodeError::OtherString("invalid header name".into()))?;
      let header_value = HeaderValue::from_bytes(&value_bytes)
        .map_err(|_| DecodeError::OtherString("invalid header value".into()))?;
      headers.append(header_name, header_value);
    }
    Ok(IndexEntry {
      offset,
      len,
      headers,
    })
  }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Index(pub(crate) HashMap<String, IndexEntry>);

impl Encode for Index {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
    self.0.encode(encoder)
  }
}

impl<T> Decode<T> for Index {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
    let map = HashMap::<String, IndexEntry>::decode(decoder)?;
    Ok(Index(map))
  }
}

impl Deref for Index {
  type Target = HashMap<String, IndexEntry>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Index {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Index {
  pub fn insert_entry<S: Into<String>>(
    &mut self,
    path: S,
    entry: IndexEntry,
  ) -> Option<IndexEntry> {
    self.0.insert(path.into(), entry)
  }

  pub fn get_entry(&self, path: &str) -> Option<&IndexEntry> {
    self.0.get(path)
  }

  pub fn get_entry_mut(&mut self, path: &str) -> Option<&mut IndexEntry> {
    self.0.get_mut(path)
  }

  pub fn remove_entry(&mut self, path: &str) -> Option<IndexEntry> {
    self.0.remove(path)
  }

  pub fn contains_path(&self, path: &str) -> bool {
    self.0.contains_key(path)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn index() {
    let mut index = Index::default();
    let mut entry = IndexEntry::new(0, 0);
    entry.headers.append(
      HeaderName::from_static("content-type"),
      HeaderValue::from_static("application/javascript"),
    );
    index.insert_entry("/index.jsx", entry);

    assert!(index.contains_path("/index.jsx"));
    assert_eq!(
      index
        .get_entry("/index.jsx")
        .map(|x| x.headers.get("content-type"))
        .unwrap(),
      Some(&HeaderValue::from_static("application/javascript"))
    );
  }
}
