#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("compress error: {0}")]
  Compress(#[from] lz4_flex::block::CompressError),
  #[error("decompress error: {0}")]
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
  // #[error("http error: {0}")]
  // Http(#[from] http::Error),
  #[error("invalid magic number")]
  InvalidMagicNum,
  #[error("invalid version format")]
  InvalidVersion,
  #[error("invalid header checksum")]
  InvalidHeaderChecksum,
  #[error("invalid index checksum")]
  InvalidIndexChecksum,
  #[error("checksum mismatch")]
  ChecksumMismatch,
}
