#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Compress(#[from] lz4_flex::block::CompressError),
  #[error(transparent)]
  Decompress(#[from] lz4_flex::block::DecompressError),
  #[error("encode error: {message}")]
  Encode {
    #[source]
    error: bincode::error::EncodeError,
    message: String,
  },
  #[error("decode error: {message}")]
  Decode {
    #[source]
    error: bincode::error::DecodeError,
    message: String,
  },
  #[error("invalid magic number")]
  InvalidMagicNum,
  #[error("invalid version format")]
  InvalidVersion,
  #[error("header checksum mismatch")]
  HeaderChecksumMismatch,
  #[error("data checksum mismatch")]
  DataChecksumMismatch,
  #[error("content checksum mismatch")]
  ContentChecksumMismatch,
  #[error("file not found")]
  FileNotFound,
}
