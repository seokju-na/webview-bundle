use base64ct::{Base64, Encoding};
use sha3::{Digest, Sha3_256, Sha3_384, Sha3_512};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Algorithm {
  Sha256,
  Sha384,
  Sha512,
}

impl Algorithm {
  pub fn digest(&self, data: &[u8]) -> Vec<u8> {
    match self {
      Self::Sha256 => Sha3_256::digest(data).to_vec(),
      Self::Sha384 => Sha3_384::digest(data).to_vec(),
      Self::Sha512 => Sha3_512::digest(data).to_vec(),
    }
  }
}

impl std::fmt::Display for Algorithm {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      Self::Sha256 => "sha256",
      Self::Sha384 => "sha384",
      Self::Sha512 => "sha512",
    };
    write!(f, "{}", str)
  }
}

impl FromStr for Algorithm {
  type Err = crate::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match &s.to_lowercase()[..] {
      "sha256" => Ok(Self::Sha256),
      "sha384" => Ok(Self::Sha384),
      "sha512" => Ok(Self::Sha512),
      _ => Err(crate::Error::invalid_integrity("invalid algorithm")),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Integrity {
  alg: Algorithm,
  value: Vec<u8>,
}

impl Integrity {
  pub fn compute(alg: Algorithm, data: &[u8]) -> Self {
    let value = alg.digest(data);
    Self { alg, value }
  }

  pub fn validate(&self, data: &[u8]) -> bool {
    self.value == self.alg.digest(data)
  }

  pub fn serialize(&self) -> String {
    let alg_str = self.alg.to_string();
    let encoded = Base64::encode_string(&self.value);
    format!("{alg_str}:{encoded}")
  }
}

impl FromStr for Integrity {
  type Err = crate::Error;

  fn from_str(str: &str) -> Result<Self, Self::Err> {
    let mut parts = str.splitn(2, ':');
    let alg_str = parts
      .next()
      .ok_or(crate::Error::invalid_integrity("algorithm is missing"))?;
    let alg = Algorithm::from_str(alg_str)?;
    let value_str = parts
      .next()
      .ok_or(crate::Error::invalid_integrity("value is missing"))?;
    let value = Base64::decode_vec(value_str)
      .map_err(|_| crate::Error::invalid_integrity("fail to decode value"))?;
    Ok(Self { alg, value })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn integrity_serialize() {
    let str = Integrity::compute(Algorithm::Sha256, b"test").serialize();
    assert_eq!(str, "sha256:NvAoWAuwLMgnKpoCD0IA40bidq5mTkXugHRVdOL1q4A=");
  }

  #[test]
  fn integrity_from_str() {
    let str = "sha256:NvAoWAuwLMgnKpoCD0IA40bidq5mTkXugHRVdOL1q4A=";
    let integrity = Integrity::from_str(str).unwrap();
    assert_eq!(integrity.alg, Algorithm::Sha256);
    assert_eq!(integrity.value, Algorithm::Sha256.digest(b"test"));
  }

  #[test]
  fn integrity_validate() {
    let integrity = Integrity::compute(Algorithm::Sha256, b"test");
    assert!(integrity.validate(b"test"));
    assert!(!integrity.validate(b"test2"));
  }
}
