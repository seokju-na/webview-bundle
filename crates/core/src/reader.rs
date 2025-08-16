use crate::bundle::{INDEX_SIZE_BYTES_LEN, MAGIC_BYTES_LEN, VERSION_BYTES_LEN};
use crate::index::{Index, IndexEntry};
use bincode::{config, decode_from_slice};
use lz4_flex::decompress_size_prepended;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

pub(crate) struct Reader {
  file_path: PathBuf,
}

impl Reader {
  pub(crate) fn read_magic_num(&self) -> crate::Result<[u8; MAGIC_BYTES_LEN]> {
    let mut file = fs::File::open(&self.file_path)?;
    let offset = 0;
    file.seek(SeekFrom::Start(offset))?;

    let mut buf = [0u8; MAGIC_BYTES_LEN];
    file.read_exact(&mut buf)?;

    Ok(buf)
  }

  pub(crate) fn read_version(&self) -> crate::Result<[u8; VERSION_BYTES_LEN]> {
    let mut file = fs::File::open(&self.file_path)?;
    let offset = MAGIC_BYTES_LEN as u64;
    (file).seek(SeekFrom::Start(offset))?;

    let mut buf = [0u8; VERSION_BYTES_LEN];
    file.read_exact(&mut buf)?;

    Ok(buf)
  }

  pub(crate) fn read_index_size(&self) -> crate::Result<u32> {
    let mut file = fs::File::open(&self.file_path)?;
    let offset = (MAGIC_BYTES_LEN + VERSION_BYTES_LEN) as u64;
    file.seek(SeekFrom::Start(offset))?;

    let mut buf = [0u8; INDEX_SIZE_BYTES_LEN];
    file.read_exact(&mut buf)?;

    let size = u32::from_be_bytes(AsRef::<[u8]>::as_ref(&buf).try_into().unwrap());
    Ok(size)
  }

  pub(crate) fn read_index(&self) -> crate::Result<Index> {
    let mut file = fs::File::open(&self.file_path)?;
    let offset = (MAGIC_BYTES_LEN + VERSION_BYTES_LEN + INDEX_SIZE_BYTES_LEN) as u64;
    file.seek(SeekFrom::Start(offset))?;

    let size = self.read_index_size()?;
    let mut buf = vec![0u8; size as usize];
    file.read_exact(&mut buf)?;

    let config = config::standard().with_big_endian();
    let (index, _): (Index, _) =
      decode_from_slice(&buf, config).map_err(|e| crate::Error::Decode {
        error: e,
        message: "fail to decoe index".to_string(),
      })?;
    Ok(index)
  }

  pub(crate) fn read_data(&self, entry: &IndexEntry) -> crate::Result<(Vec<u8>, u32)> {
    let mut file = fs::File::open(&self.file_path)?;
    file.seek(SeekFrom::Start(entry.offset() as u64))?;
    let mut buf = vec![0u8; entry.len() as usize];
    file.read_exact(&mut buf)?;
    let data = decompress_size_prepended(&buf)?;

    let offset = entry.offset() + entry.len();
    file.seek(SeekFrom::Start(offset as u64))?;
    let mut checksum_buf = [0u8; size_of::<u32>()];
    file.read_exact(&mut checksum_buf)?;
    let checksum = u32::from_be_bytes(checksum_buf);

    Ok((data, checksum))
  }
}
