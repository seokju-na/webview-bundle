use crate::builder::BundleBuilder;
use crate::checksum::{get_checksum, CHECKSUM_BYTES_LEN};
use crate::header::{Header, HeaderReader, HeaderWriter};
use crate::index::{Index, IndexEntry, IndexReader, IndexWriter};
use crate::reader::Reader;
use crate::version::Version;
use crate::writer::Writer;
use http::{header, HeaderValue, Response, StatusCode};
use lz4_flex::decompress_size_prepended;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

pub type BundleResponse = Response<Vec<u8>>;

#[derive(Debug, PartialEq, Clone)]
pub struct BundleManifest {
  header: Header,
  index: Index,
}

impl BundleManifest {
  pub fn header(&self) -> Header {
    self.header
  }

  pub fn index(&self) -> &Index {
    &self.index
  }

  pub fn reader<R: Read + Seek>(&self, reader: R) -> BundleDataReader<R> {
    BundleDataReader::new(reader)
  }

  pub fn response<R: Read + Seek>(
    &self,
    reader: R,
    path: &str,
  ) -> crate::Result<Option<BundleResponse>> {
    if !self.index.contains_path(path) {
      return Ok(None);
    }
    let entry = self.index.get_entry(path).unwrap();
    let mut reader = self.reader(reader);
    let body = reader.read_entry_data(entry)?;
    let mut response = Response::builder();
    if let Some(headers) = response.headers_mut() {
      headers.clone_from(&entry.headers);
      if !headers.contains_key(header::CONTENT_LENGTH) {
        let content_length = body.len() as u32;
        headers.append(header::CONTENT_LENGTH, HeaderValue::from(content_length));
      }
    }
    let response = response.status(StatusCode::OK).body(body)?;
    Ok(Some(response))
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bundle {
  manifest: BundleManifest,
  data: Vec<u8>,
}

impl Bundle {
  pub fn builder() -> BundleBuilder {
    BundleBuilder::new()
  }

  pub fn builder_with_capacity(capacity: usize) -> BundleBuilder {
    BundleBuilder::new_with_capacity(capacity)
  }

  pub fn manifest(&self) -> &BundleManifest {
    &self.manifest
  }

  pub fn response(&self, path: &str) -> crate::Result<Option<BundleResponse>> {
    let mut reader = Cursor::new(&self.data);
    let resp = self.manifest.response(&mut reader, path)?;
    Ok(resp)
  }
}

pub struct BundleDataReader<R: Read + Seek> {
  r: R,
}

impl<R: Read + Seek> BundleDataReader<R> {
  pub fn new(r: R) -> Self {
    Self { r }
  }

  pub fn read_entry_data(&mut self, entry: &IndexEntry) -> crate::Result<Vec<u8>> {
    let offset = entry.offset() as u64;
    let len = entry.len() as usize;
    self.r.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; len];
    self.r.read_exact(&mut buf)?;
    let decompressed = decompress_size_prepended(&buf)?;
    Ok(decompressed)
  }

  pub fn read_entry_checksum(&mut self, entry: &IndexEntry) -> crate::Result<u32> {
    let offset = entry.offset() as u64 + entry.len() as u64;
    self.r.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; CHECKSUM_BYTES_LEN];
    self.r.read_exact(&mut buf)?;
    let checksum = get_checksum(&buf);
    Ok(checksum)
  }
}

pub struct BundleReader<R: Read + Seek> {
  r: R,
}

impl<R: Read + Seek> BundleReader<R> {
  pub fn new(r: R) -> Self {
    Self { r }
  }

  pub fn read_header(&mut self) -> crate::Result<Header> {
    let mut reader = HeaderReader::new(&mut self.r);
    let header = reader.read()?;
    Ok(header)
  }

  pub fn read_index(&mut self, header: Header) -> crate::Result<Index> {
    let mut reader = IndexReader::new(&mut self.r, header);
    let index = reader.read()?;
    Ok(index)
  }

  pub fn read_data(&mut self, header: Header) -> crate::Result<Vec<u8>> {
    self.r.seek(SeekFrom::Start(header.index_end_offset()))?;
    let mut data = vec![];
    self.r.read_to_end(&mut data)?;
    Ok(data)
  }
}

impl<R: Read + Seek> Reader<BundleManifest> for BundleReader<R> {
  fn read(&mut self) -> crate::Result<BundleManifest> {
    let header = self.read_header()?;
    let index = self.read_index(header)?;
    Ok(BundleManifest { header, index })
  }
}

impl<R: Read + Seek> Reader<Bundle> for BundleReader<R> {
  fn read(&mut self) -> crate::Result<Bundle> {
    let header = self.read_header()?;
    let index = self.read_index(header)?;
    let data = self.read_data(header)?;
    Ok(Bundle {
      manifest: BundleManifest { header, index },
      data,
    })
  }
}

pub struct BundleWriter<W: Write> {
  w: W,
}

impl<W: Write> BundleWriter<W> {
  pub fn new(w: W) -> Self {
    Self { w }
  }
}

impl<W: Write> Writer<BundleBuilder> for BundleWriter<W> {
  fn write(&mut self, builder: &BundleBuilder) -> crate::Result<usize> {
    let mut index_bytes = vec![];
    let index = builder.build_index();
    let index_bytes_len = IndexWriter::new_with_options(
      Cursor::new(&mut index_bytes),
      builder.options().index_options,
    )
    .write(&index)?;
    let index_size = index_bytes_len - CHECKSUM_BYTES_LEN;

    let header = Header::new(builder.version(), index_size as u32);
    let header_len = HeaderWriter::new_with_options(&mut self.w, builder.options().header_options)
      .write(&header)?;
    self.w.write_all(&index_bytes)?;

    let data_bytes = builder.build_data();
    let data_len = data_bytes.len();
    self.w.write_all(&data_bytes)?;

    Ok(header_len + index_bytes_len + data_len)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use http::{header, HeaderMap};

  const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
  <title>test</title>
</head>
<body>
  <h1>Hello World</h1>
</body>
</html>
"#;
  const INDEX_JS: &str = r#"console.log('Hello World');"#;

  #[test]
  fn manifest() {
    let mut builder = Bundle::builder();
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
    builder.insert_entry("/index.html", (INDEX_HTML.as_bytes(), headers));
    let mut data = vec![];
    let mut writer = BundleWriter::new(Cursor::new(&mut data));
    let size = writer.write(&builder).unwrap();
    assert_eq!(size, 162);
    let mut reader = BundleReader::new(Cursor::new(&data));
    let manifest: BundleManifest = reader.read().unwrap();
    assert_eq!(manifest.header.version(), Version::Version1);
    assert_eq!(manifest.header.index_size(), 39);

    let html_entry = manifest.index.get_entry("/index.html").unwrap();
    assert_eq!(
      html_entry.headers.get(header::CONTENT_TYPE).unwrap(),
      "text/html"
    );
    assert_eq!(html_entry.offset(), 0);
    assert_eq!(html_entry.len(), 98);
  }

  #[test]
  fn responses() {
    let mut builder = Bundle::builder();
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
    builder.insert_entry("/index.html", (INDEX_HTML.as_bytes(), headers));
    builder.insert_entry("/index.js", INDEX_JS.as_bytes());
    let mut data = vec![];
    let mut writer = BundleWriter::new(Cursor::new(&mut data));
    let size = writer.write(&builder).unwrap();
    assert_eq!(size, 212);
    let mut reader = BundleReader::new(Cursor::new(&data));
    let bundle: Bundle = reader.read().unwrap();

    let html = bundle.response("/index.html").unwrap().unwrap();
    assert_eq!(html.status(), 200);
    assert_eq!(html.headers().len(), 2);
    assert_eq!(
      html.headers().get(header::CONTENT_TYPE).unwrap(),
      "text/html"
    );
    assert_eq!(html.headers().get(header::CONTENT_LENGTH).unwrap(), "106");
    assert_eq!(html.body(), INDEX_HTML.as_bytes());

    let js = bundle.response("/index.js").unwrap().unwrap();
    assert_eq!(js.status(), 200);
    assert_eq!(js.headers().len(), 1);
    assert_eq!(js.headers().get(header::CONTENT_LENGTH).unwrap(), "27");
    assert_eq!(js.body(), INDEX_JS.as_bytes());

    // Not found
    assert!(bundle.response("/not_found.html").unwrap().is_none());
  }
}
