#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Compress(#[from] lz4_flex::block::CompressError),
  #[error(transparent)]
  Decompress(#[from] lz4_flex::block::DecompressError),
  #[error(transparent)]
  Encode(#[from] bincode::error::EncodeError),
  #[error(transparent)]
  Decode(#[from] bincode::error::DecodeError),
  #[error("header magic mismatch")]
  InvalidMagic,
  #[error("invalid version format")]
  InvalidVersion,
  #[error("file not found")]
  FileNotFound,
}
