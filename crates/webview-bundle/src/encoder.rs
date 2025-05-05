use crate::bundle::{Bundle, FileDescriptors, Version, HEADER_MAGIC_BYTES};
use bincode::{config, encode_to_vec};
use std::io::Write;

pub fn encode<W: Write>(bundle: &Bundle, write: W) -> crate::Result<()> {
  Encoder::new(write).encode(bundle)?;
  Ok(())
}

pub fn encode_bytes(bundle: &Bundle) -> crate::Result<Vec<u8>> {
  let mut write = Vec::new();
  encode(bundle, &mut write)?;
  Ok(write)
}

struct Encoder<W: Write> {
  w: W,
}

impl<W: Write> Encoder<W> {
  fn new(w: W) -> Self {
    Self { w }
  }

  fn encode(&mut self, bundle: &Bundle) -> crate::Result<()> {
    self.write_magic()?;
    self.write_version(&bundle.version)?;
    self.write_file_descriptors(&bundle.descriptors)?;
    self.w.write_all(&bundle.data)?;
    Ok(())
  }

  fn write_magic(&mut self) -> crate::Result<()> {
    self.w.write_all(&HEADER_MAGIC_BYTES)?;
    Ok(())
  }

  fn write_version(&mut self, version: &Version) -> crate::Result<()> {
    self.w.write_all(version.bytes())?;
    Ok(())
  }

  fn write_file_descriptors(&mut self, descriptors: &FileDescriptors) -> crate::Result<()> {
    let mut encoded: Vec<u8> = vec![];
    let config = config::standard().with_big_endian();
    let bytes = encode_to_vec(descriptors, config).map_err(|e| crate::Error::Encode {
      error: e,
      message: "fail to encode file descriptors".to_string(),
    })?;
    let bytes_len = (bytes.len() as u32).to_be_bytes();
    encoded.extend_from_slice(&bytes_len);
    encoded.extend_from_slice(&bytes);
    self.w.write_all(&encoded)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn encode_ok() {
    let file = r#"const a = 10;"#;
    let bundle = Bundle::builder()
      .add_file("index.js", file.as_bytes())
      .build();
    let mut write = Vec::new();
    encode(&bundle, &mut write).unwrap();
    assert_eq!(
      write,
      [
        240, 159, 140, 144, 240, 159, 142, 129, 1, 0, 0, 0, 12, 1, 8, 105, 110, 100, 101, 120, 46,
        106, 115, 18, 18, 13, 0, 0, 0, 208, 99, 111, 110, 115, 116, 32, 97, 32, 61, 32, 49, 48, 59
      ]
    );
  }
}
