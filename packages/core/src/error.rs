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
  #[error("http error: {0}")]
  Http(#[from] http::Error),
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
  #[error("bundle not found")]
  BundleNotFound,
  #[cfg(feature = "_serde")]
  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),
  #[cfg(feature = "protocol-local")]
  #[error("cannot resolve local host")]
  CannotResolveLocalHost,
  #[cfg(feature = "_reqwest")]
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[cfg(feature = "remote")]
  #[error("remote http error with status {status}")]
  RemoteHttp { status: u16 },
  #[cfg(feature = "remote")]
  #[error("invalid remote config: {0}")]
  InvalidRemoteConfig(String),
  #[cfg(feature = "_opendal")]
  #[error("opendal error: {0}")]
  Opendal(#[from] opendal::Error),
}

impl Error {
  #[cfg(feature = "remote")]
  pub(crate) fn invalid_remote_config(message: impl Into<String>) -> Self {
    Self::InvalidRemoteConfig(message.into())
  }
}
