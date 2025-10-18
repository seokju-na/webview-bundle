use crate::integrity::signature;
use ed25519::pkcs8::{DecodePrivateKey, DecodePublicKey};
use ed25519::signature::{Signer, Verifier};
use ed25519_dalek::{SecretKey, SigningKey, VerifyingKey};

#[derive(Debug)]
pub struct Ed25519Signer {
  key: SigningKey,
}

impl Ed25519Signer {
  pub fn new(data: &[u8]) -> crate::Result<Self> {
    let secret = SecretKey::try_from(data).map_err(|_| crate::Error::InvalidSigningKey)?;
    let key = SigningKey::from(&secret);
    Ok(Self { key })
  }

  pub fn from_pkcs8_der(data: &[u8]) -> crate::Result<Self> {
    let key = SigningKey::from_pkcs8_der(data).map_err(|_| crate::Error::InvalidSigningKey)?;
    Ok(Self { key })
  }

  pub fn from_pkcs8_pem(data: &str) -> crate::Result<Self> {
    let key = SigningKey::from_pkcs8_pem(data).map_err(|_| crate::Error::InvalidSigningKey)?;
    Ok(Self { key })
  }
}

impl TryFrom<&[u8]> for Ed25519Signer {
  type Error = crate::Error;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    Self::new(value)
  }
}

impl signature::Signer for Ed25519Signer {
  fn sign(&self, data: &[u8]) -> Vec<u8> {
    self.key.sign(data).to_vec()
  }
}

#[derive(Debug)]
pub struct Ed25519Verifier {
  key: VerifyingKey,
}

impl Ed25519Verifier {
  pub fn new(data: &[u8]) -> crate::Result<Self> {
    let key = VerifyingKey::try_from(data).map_err(|_| crate::Error::InvalidVerifyingKey)?;
    Ok(Self { key })
  }

  pub fn from_public_key_der(data: &[u8]) -> crate::Result<Self> {
    let key =
      VerifyingKey::from_public_key_der(data).map_err(|_| crate::Error::InvalidVerifyingKey)?;
    Ok(Self { key })
  }

  pub fn from_public_key_pem(data: &str) -> crate::Result<Self> {
    let key =
      VerifyingKey::from_public_key_pem(data).map_err(|_| crate::Error::InvalidVerifyingKey)?;
    Ok(Self { key })
  }
}

impl TryFrom<&[u8]> for Ed25519Verifier {
  type Error = crate::Error;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    Self::new(value)
  }
}

impl signature::Verifier for Ed25519Verifier {
  fn verify(&self, data: &[u8], signature: &[u8]) -> crate::Result<()> {
    let signature =
      ed25519::Signature::from_slice(signature).map_err(|_| crate::Error::InvalidSignature)?;
    self
      .key
      .verify(data, &signature)
      .map_err(|_| crate::Error::VerifyFailed)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::integrity::signature::{Signer, Verifier};
  use base64ct::{Base64, Encoding};
  use std::path::PathBuf;

  #[test]
  fn sign_and_verify() {
    let key_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("ed25519");
    let private_pem = std::fs::read_to_string(key_path.join("private.pem")).unwrap();
    let public_pem = std::fs::read_to_string(key_path.join("public.pem")).unwrap();
    let signer = Ed25519Signer::from_pkcs8_pem(&private_pem).unwrap();
    let verifier = Ed25519Verifier::from_public_key_pem(&public_pem).unwrap();
    let data = b"test";
    let signature = signer.sign(data);
    assert_eq!(
      Base64::encode_string(&signature),
      "RhMuTlEJFXIfM+VZvmwet3ucxDREXLqeL8ldMo+SvwqUo6wFZdZr/CoYCfhPbE5FTSxBKTuMheWFlwaFAjpoBw=="
    );
    assert!(verifier.verify(data, &signature).is_ok());
  }
}
