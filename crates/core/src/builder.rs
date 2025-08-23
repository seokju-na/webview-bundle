use crate::checksum::{make_checksum, CHECKSUM_BYTES_LEN};
use crate::index::{Index, IndexEntry};
use http::HeaderMap;
use lz4_flex::compress_prepend_size;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct BundleEntry {
  compressed: Vec<u8>,
  len: usize,
  pub headers: Option<HeaderMap>,
}

impl BundleEntry {
  pub fn new(data: &[u8], headers: Option<HeaderMap>) -> Self {
    let compressed = compress_prepend_size(data);
    let len = compressed.len();
    Self {
      compressed,
      len,
      headers,
    }
  }

  pub fn data(&self) -> &[u8] {
    &self.compressed
  }

  pub fn len(&self) -> usize {
    self.len
  }
}

#[derive(Debug, Default)]
pub struct BundleBuilder {
  entries: HashMap<String, BundleEntry>,
}

impl BundleBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn new_with_capacity(capacity: usize) -> Self {
    Self {
      entries: HashMap::with_capacity(capacity),
    }
  }

  pub fn entries(&self) -> &HashMap<String, BundleEntry> {
    &self.entries
  }

  pub fn insert_entry<S: Into<String>, E: Into<BundleEntry>>(
    &mut self,
    path: S,
    entry: E,
  ) -> Option<BundleEntry> {
    self.entries.insert(path.into(), entry.into())
  }

  pub fn get_entry(&self, path: &str) -> Option<&BundleEntry> {
    self.entries.get(path)
  }

  pub fn get_entry_mut(&mut self, path: &str) -> Option<&mut BundleEntry> {
    self.entries.get_mut(path)
  }

  pub fn remove_entry(&mut self, path: &str) -> Option<BundleEntry> {
    self.entries.remove(path)
  }

  pub fn contains_path(&self, path: &str) -> bool {
    self.entries.contains_key(path)
  }

  pub fn build_index(&self) -> Index {
    let mut index = Index::new_with_capacity(self.entries().len());
    let mut offset = 0;
    for (path, entry) in self.entries() {
      let len = entry.len() as u32;
      let mut index_entry = IndexEntry::new(offset, len);
      if let Some(headers) = entry.headers.as_ref() {
        index_entry.headers.clone_from(headers);
      }
      index.insert_entry(path, index_entry);
      offset += len;
      offset += CHECKSUM_BYTES_LEN as u32;
    }
    index
  }

  pub fn build_data(&self) -> Vec<u8> {
    let mut data = vec![];
    for entry in self.entries().values() {
      let checksum = make_checksum(entry.data());
      data.extend_from_slice(entry.data());
      data.extend_from_slice(&checksum.to_be_bytes());
    }
    data
  }
}
