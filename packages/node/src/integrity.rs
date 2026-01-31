use napi_derive::napi;
use wvb::integrity;

#[napi(string_enum = "camelCase")]
pub enum IntegrityAlgorithm {
  Sha256,
  Sha384,
  Sha512,
}

impl From<integrity::IntegrityAlgorithm> for IntegrityAlgorithm {
  fn from(value: integrity::IntegrityAlgorithm) -> Self {
    match value {
      integrity::IntegrityAlgorithm::Sha256 => Self::Sha256,
      integrity::IntegrityAlgorithm::Sha384 => Self::Sha384,
      integrity::IntegrityAlgorithm::Sha512 => Self::Sha512,
    }
  }
}

impl From<IntegrityAlgorithm> for integrity::IntegrityAlgorithm {
  fn from(value: IntegrityAlgorithm) -> Self {
    match value {
      IntegrityAlgorithm::Sha256 => integrity::IntegrityAlgorithm::Sha256,
      IntegrityAlgorithm::Sha384 => integrity::IntegrityAlgorithm::Sha384,
      IntegrityAlgorithm::Sha512 => integrity::IntegrityAlgorithm::Sha512,
    }
  }
}

#[napi(string_enum = "camelCase")]
pub enum IntegrityPolicy {
  Strict,
  Optional,
  None,
}

impl From<integrity::IntegrityPolicy> for IntegrityPolicy {
  fn from(value: integrity::IntegrityPolicy) -> Self {
    match value {
      integrity::IntegrityPolicy::Strict => Self::Strict,
      integrity::IntegrityPolicy::Optional => Self::Optional,
      integrity::IntegrityPolicy::None => Self::None,
    }
  }
}

impl From<IntegrityPolicy> for integrity::IntegrityPolicy {
  fn from(value: IntegrityPolicy) -> Self {
    match value {
      IntegrityPolicy::Strict => Self::Strict,
      IntegrityPolicy::Optional => Self::Optional,
      IntegrityPolicy::None => Self::None,
    }
  }
}
