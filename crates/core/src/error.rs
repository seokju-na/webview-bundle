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
  #[error(transparent)]
  Http(#[from] http::Error),
  #[error("invalid magic number")]
  InvalidMagicNum,
  #[error("invalid version format")]
  InvalidVersion,
  #[error("invalid checksum")]
  InvalidChecksum,
  #[error("checksum mismatch")]
  ChecksumMismatch,
}
