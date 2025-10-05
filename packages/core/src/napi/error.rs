use napi::bindgen_prelude::*;

impl From<crate::Error> for Error {
  fn from(value: crate::Error) -> Self {
    match value {
      crate::Error::Encode { message, .. } => Error::new(Status::GenericFailure, message),
      crate::Error::Decode { message, .. } => Error::new(Status::GenericFailure, message),
      crate::Error::InvalidMagicNum
      | crate::Error::InvalidVersion
      | crate::Error::InvalidHeaderChecksum
      | crate::Error::InvalidIndexChecksum
      | crate::Error::ChecksumMismatch
      | crate::Error::Http(_)
      | crate::Error::Io(_)
      | crate::Error::Compress(_)
      | crate::Error::Decompress(_)
      | crate::Error::BundleNotFound => Error::new(Status::GenericFailure, value.to_string()),
      #[cfg(feature = "_serde")]
      crate::Error::Json(_) => Error::new(Status::GenericFailure, value.to_string()),
      #[cfg(feature = "protocol-local")]
      crate::Error::CannotResolveLocalHost => Error::new(Status::GenericFailure, value.to_string()),
      #[cfg(feature = "_reqwest")]
      crate::Error::Reqwest(_) => Error::new(Status::GenericFailure, value.to_string()),
      #[cfg(feature = "remote")]
      crate::Error::RemoteHttp { .. } => Error::new(Status::GenericFailure, value.to_string()),
      #[cfg(feature = "remote")]
      crate::Error::InvalidRemoteConfig(_) => Error::new(Status::GenericFailure, value.to_string()),
      #[cfg(feature = "_opendal")]
      crate::Error::Opendal(_) => Error::new(Status::GenericFailure, value.to_string()),
      crate::Error::Napi(e) => e,
    }
  }
}

impl From<crate::Error> for JsError {
  fn from(value: crate::Error) -> Self {
    napi::JsError::from(napi::Error::from(value))
  }
}
