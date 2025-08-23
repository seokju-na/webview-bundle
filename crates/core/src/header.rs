use crate::checksum::{get_checksum, make_checksum, CHECKSUM_BYTES_LEN};
use crate::reader::Reader;
use crate::version::{Version, VERSION_BYTES_LEN};
use crate::writer::Writer;
use std::io::{Read, Seek, SeekFrom, Write};

const MAGIC_BYTES_LEN: usize = 8;
const MAGIC_BYTES: [u8; MAGIC_BYTES_LEN] = [0xf0, 0x9f, 0x8c, 0x90, 0xf0, 0x9f, 0x8e, 0x81];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Header {
  magic: [u8; MAGIC_BYTES_LEN],
  version: Version,
  index_size: u32,
}

impl Header {
  pub const MAGIC_OFFSET: u64 = 0;
  pub const VERSION_OFFSET: u64 = MAGIC_BYTES_LEN as u64;
  pub const INDEX_SIZE_OFFSET: u64 = Self::VERSION_OFFSET + VERSION_BYTES_LEN as u64;
  pub const INDEX_SIZE_BYTES_LEN: usize = 4;
  pub const CHECKSUM_OFFSET: u64 = Self::INDEX_SIZE_OFFSET + Self::INDEX_SIZE_BYTES_LEN as u64;
  pub const END_OFFSET: u64 = Self::CHECKSUM_OFFSET + CHECKSUM_BYTES_LEN as u64;

  pub fn header_end_offset(&self) -> u64 {
    Self::END_OFFSET
  }

  pub fn index_end_offset(&self) -> u64 {
    self.header_end_offset() + self.index_size as u64 + CHECKSUM_BYTES_LEN as u64
  }

  pub fn new(version: Version, index_size: u32) -> Self {
    Self {
      magic: MAGIC_BYTES,
      version,
      index_size,
    }
  }

  pub fn version(&self) -> Version {
    self.version
  }

  pub fn index_size(&self) -> u32 {
    self.index_size
  }
}

pub struct HeaderWriter<W: Write> {
  w: W,
}

impl<W: Write> HeaderWriter<W> {
  pub fn new(w: W) -> Self {
    Self { w }
  }

  pub fn write_magic(&mut self) -> crate::Result<Vec<u8>> {
    let bytes = MAGIC_BYTES.to_vec();
    self.w.write_all(&bytes)?;
    Ok(bytes)
  }

  pub fn write_version(&mut self, version: Version) -> crate::Result<Vec<u8>> {
    let bytes = version.bytes().to_vec();
    self.w.write_all(&bytes)?;
    Ok(bytes)
  }

  pub fn write_index_size(&mut self, index_size: u32) -> crate::Result<Vec<u8>> {
    let bytes = index_size.to_be_bytes().to_vec();
    self.w.write_all(&bytes)?;
    Ok(bytes)
  }

  pub fn write_checksum(&mut self, checksum: u32) -> crate::Result<Vec<u8>> {
    let bytes = checksum.to_be_bytes().to_vec();
    self.w.write_all(&bytes)?;
    Ok(bytes)
  }
}

impl<W: Write> Writer<Header> for HeaderWriter<W> {
  fn write(&mut self, header: &Header) -> crate::Result<usize> {
    let mut bytes = vec![];
    bytes.extend(self.write_magic()?);
    bytes.extend(self.write_version(header.version)?);
    bytes.extend(self.write_index_size(header.index_size)?);

    let checksum = make_checksum(&bytes);
    self.write_checksum(checksum)?;
    Ok(bytes.len())
  }
}

pub struct HeaderReader<R: Read + Seek> {
  r: R,
  options: HeaderReaderOptions,
}

pub struct HeaderReaderOptions {
  pub verify_checksum: bool,
}

impl Default for HeaderReaderOptions {
  fn default() -> Self {
    Self {
      verify_checksum: true,
    }
  }
}

impl<R: Read + Seek> HeaderReader<R> {
  pub fn new(r: R) -> Self {
    Self::new_with_options(r, Default::default())
  }

  pub fn new_with_options(r: R, options: HeaderReaderOptions) -> Self {
    Self { r, options }
  }

  pub fn read_magic(&mut self) -> crate::Result<[u8; MAGIC_BYTES_LEN]> {
    self.r.seek(SeekFrom::Start(Header::MAGIC_OFFSET))?;
    let mut buf = [0u8; MAGIC_BYTES_LEN];
    self.r.read_exact(&mut buf)?;
    if buf != MAGIC_BYTES {
      return Err(crate::Error::InvalidMagicNum);
    }
    Ok(buf)
  }

  pub fn read_version(&mut self) -> crate::Result<Version> {
    self.r.seek(SeekFrom::Start(Header::VERSION_OFFSET))?;
    let mut buf = [0u8; VERSION_BYTES_LEN];
    self.r.read_exact(&mut buf)?;
    if buf == Version::Version1.bytes() {
      return Ok(Version::Version1);
    }
    Err(crate::Error::InvalidVersion)
  }

  pub fn read_index_size(&mut self) -> crate::Result<u32> {
    self.r.seek(SeekFrom::Start(Header::INDEX_SIZE_OFFSET))?;
    let mut buf = [0u8; Header::INDEX_SIZE_BYTES_LEN];
    self.r.read_exact(&mut buf)?;
    let size = u32::from_be_bytes(AsRef::<[u8]>::as_ref(&buf).try_into().unwrap());
    Ok(size)
  }

  pub fn read_checksum(&mut self) -> crate::Result<u32> {
    self.r.seek(SeekFrom::Start(Header::CHECKSUM_OFFSET))?;
    let mut buf = vec![0u8; CHECKSUM_BYTES_LEN];
    self.r.read_exact(&mut buf)?;
    let checksum = get_checksum(&buf);
    Ok(checksum)
  }

  fn verify_checksum(&mut self, checksum: u32) -> crate::Result<()> {
    self.r.seek(SeekFrom::Start(Header::MAGIC_OFFSET))?;

    let total_len = Header::CHECKSUM_OFFSET;
    let mut total = vec![0u8; total_len as usize];
    self.r.read_exact(&mut total)?;

    let expected_checksum = make_checksum(&total);
    if checksum != expected_checksum {
      return Err(crate::Error::InvalidChecksum);
    }
    Ok(())
  }
}

impl<R: Read + Seek> Reader<Header> for HeaderReader<R> {
  fn read(&mut self) -> crate::Result<Header> {
    self.read_magic()?;
    let version = self.read_version()?;
    let index_size = self.read_index_size()?;
    let checksum = self.read_checksum()?;
    if self.options.verify_checksum {
      self.verify_checksum(checksum)?;
    }
    Ok(Header::new(version, index_size))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Cursor;

  #[test]
  fn read_and_write() {
    let header = Header::new(Version::Version1, 1234);
    let mut buf = vec![];
    let mut writer = HeaderWriter::new(Cursor::new(&mut buf));
    writer.write(&header).unwrap();
    assert_eq!(
      buf,
      [240, 159, 140, 144, 240, 159, 142, 129, 1, 0, 0, 4, 210, 49, 56, 3, 16]
    );
    let mut reader = HeaderReader::new(Cursor::new(&buf));
    let read_header = reader.read().unwrap();
    assert_eq!(header, read_header);
    assert_eq!(read_header.version(), Version::Version1);
    assert_eq!(read_header.index_size(), 1234);
  }
}
