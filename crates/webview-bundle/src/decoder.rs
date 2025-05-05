use std::hash::Hasher;
use crate::bundle::{
  Bundle, FileDescriptors, Version, FILE_DESCRIPTORS_SIZE_BYTES_LENGTH, HEADER_MAGIC_BYTES,
  VERSION_BYTES_LENGTH,
};
use bincode::{config, decode_from_slice};
use std::io::{Cursor, Read};
use twox_hash::XxHash32;

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
      return Err(crate::Error::InvalidMagicNum);
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
  
  fn read_header_checksum(&mut self) -> crate::Result<u32> {
    let mut buf = [0; 4];
    self.c.read_exact(&mut buf)?;

    let mut hasher = XxHash32::with_seed(0);
    hasher.write(&original_input[0..original_input.len() - input.len() - 1]);
  }

  fn read_file_descriptors(&mut self) -> crate::Result<FileDescriptors> {
    let mut size_buf = [0; FILE_DESCRIPTORS_SIZE_BYTES_LENGTH];
    self.c.read_exact(&mut size_buf)?;
    let size = u32::from_be_bytes(AsRef::<[u8]>::as_ref(&size_buf).try_into().unwrap());

    let mut descriptors_buf = vec![0; size as usize];
    self.c.read_exact(&mut descriptors_buf)?;
    let config = config::standard().with_big_endian();
    let (descriptors, _): (FileDescriptors, _) =
      decode_from_slice(&descriptors_buf, config).map_err(|e| crate::Error::Decode {
        error: e,
        message: "fail to decode file descriptors".to_string(),
      })?;
    Ok(descriptors)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::encoder::encode_bytes;

  #[test]
  fn encode_and_decode() {
    let file = r#"const a = 10;"#;
    let bundle = Bundle::builder()
      .add_file("index.js", file.as_bytes())
      .build();
    let encoded = encode_bytes(&bundle).unwrap();
    let decoded = decode(encoded).unwrap();
    assert_eq!(bundle, decoded);
  }

  #[test]
  fn invalid_magic() {
    assert!(matches!(
      decode(vec![0, 0, 0, 0, 0, 0, 0, 0]).unwrap_err(),
      crate::Error::InvalidMagicNum,
    ));
  }
}
