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
  #[error("serde json error: {0}")]
  SerdeJson(#[from] serde_json::Error),
  #[cfg(feature = "protocol-local")]
  #[error("cannot resolve local host")]
  CannotResolveLocalHost,
  #[cfg(feature = "_reqwest")]
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[cfg(feature = "remote")]
  #[error("invalid remote bundle: {0}")]
  InvalidRemoteBundle(String),
  #[cfg(feature = "remote")]
  #[error("remote bundle not found")]
  RemoteBundleNotFound,
  #[cfg(feature = "remote")]
  #[error("remote forbidden")]
  RemoteForbidden,
  #[cfg(feature = "remote")]
  #[error("remote http error with status {status}")]
  RemoteHttp {
    status: u16,
    message: Option<String>,
  },
  #[cfg(feature = "remote")]
  #[error("invalid remote config: {0}")]
  InvalidRemoteConfig(String),
  #[cfg(feature = "_opendal")]
  #[error("opendal error: {0}")]
  Opendal(#[from] opendal::Error),
  #[cfg(feature = "integrity")]
  #[error("invalid integrity: {0}")]
  InvalidIntegrity(String),
  #[cfg(feature = "integrity")]
  #[error("integrity verify failed")]
  IntegrityVerifyFailed,
  #[error("unknown error: {0}")]
  Unknown(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl Error {
  #[cfg(feature = "remote")]
  pub(crate) fn invalid_remote_config(message: impl Into<String>) -> Self {
    Self::InvalidRemoteConfig(message.into())
  }

  #[cfg(feature = "remote")]
  pub(crate) fn invalid_remote_bundle(message: impl Into<String>) -> Self {
    Self::InvalidRemoteBundle(message.into())
  }

  #[cfg(feature = "remote")]
  pub(crate) fn remote_http(status: http::StatusCode, message: Option<impl Into<String>>) -> Self {
    Self::RemoteHttp {
      status: status.as_u16(),
      message: message.map(|x| x.into()),
    }
  }

  #[cfg(feature = "integrity")]
  pub(crate) fn invalid_integrity(message: impl Into<String>) -> Self {
    Self::InvalidIntegrity(message.into())
  }

  pub(crate) fn unknown(
    error: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  ) -> Self {
    Self::Unknown(error.into())
  }
}
