use crate::bundle::{Bundle, HEADER_MAGIC_BYTES};
use bincode::{config, encode_to_vec};
use std::hash::Hasher;
use std::io::Write;
use twox_hash::XxHash32;

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
    let mut data = vec![];
    data.extend(self.write_magic()?);
    data.extend(self.write_version(&bundle)?);
    data.extend(self.write_file_descriptors(&bundle)?);

    let header_checksum = self.write_checksum(&data)?;
    data.extend_from_slice(header_checksum.to_be_bytes().as_ref());

    self.w.write_all(&bundle.data)?;
    data.extend_from_slice(&bundle.data);
    self.write_checksum(&data)?;

    Ok(())
  }

  fn write_magic(&mut self) -> crate::Result<Vec<u8>> {
    let bytes = HEADER_MAGIC_BYTES.to_vec();
    self.w.write_all(&bytes)?;
    Ok(bytes)
  }

  fn write_version(&mut self, bundle: &Bundle) -> crate::Result<Vec<u8>> {
    let bytes = bundle.version().bytes().to_vec();
    self.w.write_all(&bytes)?;
    Ok(bytes)
  }

  fn write_file_descriptors(&mut self, bundle: &Bundle) -> crate::Result<Vec<u8>> {
    let mut encoded: Vec<u8> = vec![];
    let config = config::standard().with_big_endian();
    let bytes = encode_to_vec(bundle.descriptors(), config).map_err(|e| crate::Error::Encode {
      error: e,
      message: "fail to encode file descriptors".to_string(),
    })?;
    let bytes_len = (bytes.len() as u32).to_be_bytes();
    encoded.extend_from_slice(&bytes_len);
    encoded.extend_from_slice(&bytes);
    self.w.write_all(&encoded)?;
    Ok(encoded)
  }

  fn write_checksum(&mut self, data: &[u8]) -> crate::Result<u32> {
    let mut hasher = XxHash32::with_seed(0);
    hasher.write(data);
    let checksum = hasher.finish() as u32;
    let checksum_bytes = checksum.to_be_bytes();
    self.w.write_all(&checksum_bytes)?;
    Ok(checksum)
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
        106, 115, 0, 18, 57, 202, 208, 23, 13, 0, 0, 0, 208, 99, 111, 110, 115, 116, 32, 97, 32,
        61, 32, 49, 48, 59, 22, 138, 45, 182
      ]
    );
  }
}
