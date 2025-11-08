use crate::signature::{Signer as SignatureSigner, Verifier as SignatureVerifier};
use crate::Bundle;
use base64ct::{Base64, Encoding};
use p256::ecdsa::signature::{Signer, Verifier};
use p256::ecdsa::{Signature, SigningKey, VerifyingKey};
use p256::pkcs8::{DecodePrivateKey, DecodePublicKey};
use p256::SecretKey;

pub struct EcdsaSecp256r1Signer {
  key: SigningKey,
}

impl EcdsaSecp256r1Signer {
  pub fn from_slice(slice: &[u8]) -> crate::Result<Self> {
    let key: SigningKey = SecretKey::from_slice(slice)
      .map(Into::into)
      .map_err(crate::Error::invalid_signing_key)?;
    Ok(Self { key })
  }

  pub fn from_sec1_der(der_bytes: &[u8]) -> crate::Result<Self> {
    let key: SigningKey = SecretKey::from_sec1_der(der_bytes)
      .map(Into::into)
      .map_err(crate::Error::invalid_signing_key)?;
    Ok(Self { key })
  }

  pub fn from_sec1_pem(sec1_pem: &str) -> crate::Result<Self> {
    let key: SigningKey = SecretKey::from_sec1_pem(sec1_pem)
      .map(Into::into)
      .map_err(crate::Error::invalid_signing_key)?;
    Ok(Self { key })
  }

  pub fn from_pkcs8_pem(pkcs8_pem: &str) -> crate::Result<Self> {
    let key: SigningKey = SecretKey::from_pkcs8_pem(pkcs8_pem)
      .map(Into::into)
      .map_err(crate::Error::invalid_signing_key)?;
    Ok(Self { key })
  }
}

impl SignatureSigner for EcdsaSecp256r1Signer {
  async fn sign(&self, _bundle: &Bundle, data: &[u8]) -> crate::Result<String> {
    let signature: Signature = self
      .key
      .try_sign(data)
      .map_err(crate::Error::signature_sign_failed)?;
    let encoded = Base64::encode_string(&signature.to_bytes());
    Ok(encoded)
  }
}

pub struct EcdsaSecp256r1Verifier {
  key: VerifyingKey,
}

impl EcdsaSecp256r1Verifier {
  pub fn from_sec1_bytes(bytes: &[u8]) -> crate::Result<Self> {
    let key = VerifyingKey::from_sec1_bytes(bytes).map_err(crate::Error::invalid_verifying_key)?;
    Ok(Self { key })
  }

  pub fn from_public_key_der(bytes: &[u8]) -> crate::Result<Self> {
    let key =
      VerifyingKey::from_public_key_der(bytes).map_err(crate::Error::invalid_verifying_key)?;
    Ok(Self { key })
  }

  pub fn from_public_key_pem(pem: &str) -> crate::Result<Self> {
    let key =
      VerifyingKey::from_public_key_pem(pem).map_err(crate::Error::invalid_verifying_key)?;
    Ok(Self { key })
  }
}

impl SignatureVerifier for EcdsaSecp256r1Verifier {
  async fn verify(&self, signature: &str, _bundle: &Bundle, data: &[u8]) -> crate::Result<bool> {
    let signature =
      Signature::from_slice(signature.as_bytes()).map_err(|_| crate::Error::InvalidSignature)?;
    Ok(self.key.verify(data, &signature).is_ok())
  }
}
