use crate::checksum::{get_checksum, CHECKSUM_BYTES_LEN};
use crate::header::Header;
use crate::index::{Index, IndexEntry};
use http::Response;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, PartialEq, Clone)]
pub struct Bundle {
  header: Header,
  index: Index,
}

impl Bundle {
  pub fn response<R: Read + Seek>(
    &self,
    reader: R,
    path: &str,
  ) -> crate::Result<Option<Response<Vec<u8>>>> {
    if !self.index.contains_path(path) {
      return Ok(None);
    }
    let entry = self.index.get_entry(path).unwrap();
    let mut reader = BundleDataReader::new(reader, self.header);
    let body = reader.read_entry(entry)?;
    let mut response = Response::builder();
    if let Some(headers) = response.headers_mut() {
      headers.clone_from(&entry.headers);
    }
    let response = response.status(200).body(body)?;
    Ok(Some(response))
  }
}

pub struct BundleDataReader<R: Read + Seek> {
  r: R,
  header: Header,
}

impl<R: Read + Seek> BundleDataReader<R> {
  pub fn new(r: R, header: Header) -> Self {
    Self { r, header }
  }

  pub fn read_entry(&mut self, entry: &IndexEntry) -> crate::Result<Vec<u8>> {
    let offset = Header::end_offset() + self.header.index_size() as u64 + entry.offset() as u64;
    self.r.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; entry.len() as usize];
    self.r.read_exact(&mut buf)?;
    Ok(buf)
  }

  pub fn read_entry_checksum(&mut self, entry: &IndexEntry) -> crate::Result<u32> {
    let offset = Header::end_offset()
      + self.header.index_size() as u64
      + entry.offset() as u64
      + entry.len() as u64;
    self.r.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; CHECKSUM_BYTES_LEN];
    self.r.read_exact(&mut buf)?;
    let checksum = get_checksum(&buf);
    Ok(checksum)
  }
}
