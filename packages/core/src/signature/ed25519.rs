use crate::signature::{Signer as SignatureSigner, Verifier as SignatureVerifier};
use crate::Bundle;
use base64ct::{Base64, Encoding};
use ed25519_dalek::pkcs8::DecodePrivateKey;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::future::Future;

pub struct Ed25519Signer {
  key: SigningKey,
}

impl Ed25519Signer {
  pub fn from_keypair_bytes(bytes: &[u8; 64]) -> crate::Result<Self> {
    let key = SigningKey::from_keypair_bytes(bytes).map_err(crate::Error::invalid_signing_key)?;
    Ok(Self { key })
  }

  pub fn from_pkcs8_pem(pem: &str) -> crate::Result<Self> {
    let key = SigningKey::from_pkcs8_pem(pem).map_err(crate::Error::invalid_signing_key)?;
    Ok(Self { key })
  }
}

impl SignatureSigner for Ed25519Signer {
  async fn sign(&self, _bundle: &Bundle, data: &[u8]) -> crate::Result<String> {
    let signature = self
      .key
      .try_sign(data)
      .map_err(crate::Error::signature_sign_failed)?;
    let encoded = Base64::encode_string(&signature.to_bytes());
    Ok(encoded)
  }
}

pub struct Ed25519Verifier {
  key: VerifyingKey,
}

impl SignatureVerifier for Ed25519Verifier {
  async fn verify(&self, signature: &str, _bundle: &Bundle, data: &[u8]) -> crate::Result<bool> {
    let signature =
      Signature::from_slice(signature.as_bytes()).map_err(|_| crate::Error::InvalidSignature)?;
    Ok(self.key.verify(data, &signature).is_ok())
  }
}
