use base64ct::{Base64, Encoding};
use serde::de::IntoDeserializer;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha3::{Digest, Sha3_256, Sha3_384, Sha3_512};

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

impl Serialize for Algorithm {
  fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    s.serialize_str(&self.to_string())
  }
}

impl<'de> Deserialize<'de> for Algorithm {
  fn deserialize<D>(d: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let str = String::deserialize(d)?;
    match &str.to_lowercase()[..] {
      "sha256" => Ok(Self::Sha256),
      "sha384" => Ok(Self::Sha384),
      "sha512" => Ok(Self::Sha512),
      _ => Err(serde::de::Error::custom("invalid algorithm")),
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
}

impl Serialize for Integrity {
  fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let alg_str = self.alg.to_string();
    let encoded = Base64::encode_string(&self.value);
    s.serialize_str(&format!("{alg_str}:{encoded}"))
  }
}

impl<'de> Deserialize<'de> for Integrity {
  fn deserialize<D>(d: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let str = String::deserialize(d)?;
    let mut parts = str.splitn(2, ':');
    let alg_str = parts.next().ok_or(serde::de::Error::custom(
      "invalid integrity: algorithm is missing",
    ))?;
    let alg = Algorithm::deserialize(alg_str.into_deserializer())?;
    let value_str = parts.next().ok_or(serde::de::Error::custom(
      "invalid integrity: value is missing",
    ))?;
    let value = Base64::decode_vec(value_str)
      .map_err(|_| serde::de::Error::custom("fail to decode integrity value"))?;
    Ok(Self { alg, value })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn algorithm_serialize() {
    let str = serde_json::to_string(&Algorithm::Sha256).unwrap();
    assert_eq!(str, "\"sha256\"");
    let str = serde_json::to_string(&Algorithm::Sha384).unwrap();
    assert_eq!(str, "\"sha384\"");
    let str = serde_json::to_string(&Algorithm::Sha512).unwrap();
    assert_eq!(str, "\"sha512\"");
  }

  #[test]
  fn algorithm_deserialize() {
    let str = r#""sha256""#;
    let alg: Algorithm = serde_json::from_str(str).unwrap();
    assert_eq!(alg, Algorithm::Sha256);
    let str = r#""sha384""#;
    let alg: Algorithm = serde_json::from_str(str).unwrap();
    assert_eq!(alg, Algorithm::Sha384);
    let str = r#""sha512""#;
    let alg: Algorithm = serde_json::from_str(str).unwrap();
    assert_eq!(alg, Algorithm::Sha512);
    // case-insensitive
    let str = r#""SHA256""#;
    let alg: Algorithm = serde_json::from_str(str).unwrap();
    assert_eq!(alg, Algorithm::Sha256);
  }

  #[test]
  fn integrity_serialize() {
    let str = serde_json::to_string(&Integrity::compute(Algorithm::Sha256, b"test")).unwrap();
    assert_eq!(
      str,
      "\"sha256:NvAoWAuwLMgnKpoCD0IA40bidq5mTkXugHRVdOL1q4A=\""
    );
  }

  #[test]
  fn integrity_deserialize() {
    let str = "\"sha256:NvAoWAuwLMgnKpoCD0IA40bidq5mTkXugHRVdOL1q4A=\"";
    let integrity: Integrity = serde_json::from_str(str).unwrap();
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
