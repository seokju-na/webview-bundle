use crate::builder::BundleBuilder;
use crate::checksum::{get_checksum, CHECKSUM_BYTES_LEN};
use crate::header::{Header, HeaderReader, HeaderWriter};
use crate::index::{Index, IndexEntry, IndexReader, IndexWriter};
use crate::reader::Reader;
use crate::version::Version;
use crate::writer::Writer;
use http::Response;
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
    BundleDataReader::new(reader, self.header)
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
    let body = reader.read_entry(entry)?;
    let mut response = Response::builder();
    if let Some(headers) = response.headers_mut() {
      headers.clone_from(&entry.headers);
    }
    let response = response.status(200).body(body)?;
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

struct BundleDataReader<R: Read + Seek> {
  r: R,
  header: Header,
}

impl<R: Read + Seek> BundleDataReader<R> {
  pub fn new(r: R, header: Header) -> Self {
    Self { r, header }
  }

  pub fn read_entry(&mut self, entry: &IndexEntry) -> crate::Result<Vec<u8>> {
    self
      .r
      .seek(SeekFrom::Start(self.header.index_end_offset()))?;
    let mut buf = vec![0u8; entry.len() as usize];
    self.r.read_exact(&mut buf)?;
    Ok(buf)
  }

  pub fn read_entry_checksum(&mut self, entry: &IndexEntry) -> crate::Result<u32> {
    let offset = self.header.index_end_offset() + entry.offset() as u64 + entry.len() as u64;
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
    let index_size = IndexWriter::new(Cursor::new(&mut index_bytes)).write(&index)?;

    let header = Header::new(Version::Version1, index_size as u32);
    let header_size = HeaderWriter::new(&mut self.w).write(&header)?;
    self.w.write_all(&index_bytes)?;

    let data_bytes = builder.build_data();
    let data_size = data_bytes.len();
    self.w.write_all(&data_bytes)?;

    Ok(header_size + index_size + data_size)
  }
}
