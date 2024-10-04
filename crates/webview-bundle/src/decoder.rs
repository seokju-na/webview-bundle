use crate::bundle::{
  Bundle, FileDescriptor, Version, FILE_DESCRIPTORS_SIZE_BYTES_LENGTH, HEADER_MAGIC_BYTES,
  VERSION_BYTES_LENGTH,
};
use bincode::{config, decode_from_slice};
use std::io::{Cursor, Read};

pub fn decode(buf: impl AsRef<[u8]>) -> crate::Result<Bundle> {
  Decoder::new(&buf).decode()
}

struct Decoder<T> {
  c: Cursor<T>,
}

impl<T> Decoder<T> {
  fn new(buf: T) -> Self {
    Self {
      c: Cursor::new(buf),
    }
  }
}

impl<T: AsRef<[u8]>> Decoder<T> {
  fn decode(&mut self) -> crate::Result<Bundle> {
    // TODO: check checksum?
    self.read_magic_bytes()?;
    let version = self.read_version()?;
    let descriptors = self.read_file_descriptors()?;
    let mut data = Vec::new();
    self.c.read_to_end(&mut data)?;
    let bundle = Bundle {
      version,
      descriptors,
      data,
    };
    Ok(bundle)
  }

  fn read_magic_bytes(&mut self) -> crate::Result<()> {
    let mut buf = [0; HEADER_MAGIC_BYTES.len()];
    self.c.read_exact(&mut buf)?;
    if buf != HEADER_MAGIC_BYTES {
      return Err(crate::Error::InvalidMagic);
    }
    Ok(())
  }

  fn read_version(&mut self) -> crate::Result<Version> {
    let mut buf = [0; VERSION_BYTES_LENGTH];
    self.c.read_exact(&mut buf)?;
    if &buf == Version::Version1.bytes() {
      return Ok(Version::Version1);
    }
    Err(crate::Error::InvalidVersion)
  }

  fn read_file_descriptors(&mut self) -> crate::Result<Vec<FileDescriptor>> {
    let mut size_buf = [0; FILE_DESCRIPTORS_SIZE_BYTES_LENGTH];
    self.c.read_exact(&mut size_buf)?;
    let size = u32::from_be_bytes(AsRef::<[u8]>::as_ref(&size_buf).try_into().unwrap());

    let mut descriptors_buf = vec![0; size as usize];
    self.c.read_exact(&mut descriptors_buf)?;
    let config = config::standard().with_big_endian();
    let (file_descriptors, _): (Vec<FileDescriptor>, _) =
      decode_from_slice(&descriptors_buf, config)?;
    Ok(file_descriptors)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::encoder::encode_bytes;
  use std::path::Path;

  #[test]
  fn encode_and_decode() {
    let path = Path::new("index.js");
    let file = r#"const a = 10;"#;
    let bundle = Bundle::builder().add_file(path, file.as_bytes()).build();
    let encoded = encode_bytes(&bundle).unwrap();
    let decoded = decode(encoded).unwrap();
    assert_eq!(bundle, decoded);
  }

  #[test]
  fn invalid_magic() {
    assert!(matches!(
      decode(vec![0, 0, 0, 0, 0, 0, 0, 0]).unwrap_err(),
      crate::Error::InvalidMagic,
    ));
  }
}
