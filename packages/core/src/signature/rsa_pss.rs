use crate::signature::{Signer as SignatureSigner, Verifier as SignatureVerifier};
use crate::Bundle;
use base64ct::{Base64, Encoding};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::pss::{Signature, SigningKey, VerifyingKey};
use rsa::sha2::Sha256;
use rsa::signature::{Keypair, SignatureEncoding, Signer, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct RsaPssSigner {
  key: SigningKey<Sha256>,
}

impl RsaPssSigner {
  pub fn from_pkcs1_der(bytes: &[u8]) -> crate::Result<Self> {
    let key = RsaPrivateKey::from_pkcs1_der(bytes).map_err(crate::Error::invalid_signing_key)?;
    let key = SigningKey::<Sha256>::new(key);
    Ok(Self { key })
  }

  pub fn from_pkcs1_pem(pem: &str) -> crate::Result<Self> {
    let key = RsaPrivateKey::from_pkcs1_pem(pem).map_err(crate::Error::invalid_signing_key)?;
    let key = SigningKey::<Sha256>::new(key);
    Ok(Self { key })
  }

  pub fn from_pkcs8_der(bytes: &[u8]) -> crate::Result<Self> {
    let key = RsaPrivateKey::from_pkcs8_der(bytes).map_err(crate::Error::invalid_signing_key)?;
    let key = SigningKey::<Sha256>::new(key);
    Ok(Self { key })
  }

  pub fn from_pkcs8_pem(pem: &str) -> crate::Result<Self> {
    let key = RsaPrivateKey::from_pkcs8_pem(pem).map_err(crate::Error::invalid_signing_key)?;
    let key = SigningKey::<Sha256>::new(key);
    Ok(Self { key })
  }
}

impl SignatureSigner for RsaPssSigner {
  async fn sign(&self, _bundle: &Bundle, data: &[u8]) -> crate::Result<String> {
    let signature = self
      .key
      .try_sign(data)
      .map_err(crate::Error::signature_sign_failed)?;
    let encoded = Base64::encode_string(&signature.to_vec());
    Ok(encoded)
  }
}

pub struct RsaPssVerifier {
  key: VerifyingKey<Sha256>,
}

impl RsaPssVerifier {
  pub fn from_signer(signer: &RsaPssSigner) -> Self {
    let key = signer.key.verifying_key();
    Self { key }
  }

  pub fn from_public_key_der(bytes: &[u8]) -> crate::Result<Self> {
    let public_key =
      RsaPublicKey::from_public_key_der(bytes).map_err(crate::Error::invalid_verifying_key)?;
    let key = VerifyingKey::<Sha256>::from(public_key);
    Ok(Self { key })
  }

  pub fn from_public_key_pem(pem: &str) -> crate::Result<Self> {
    let public_key =
      RsaPublicKey::from_public_key_pem(pem).map_err(crate::Error::invalid_verifying_key)?;
    let key = VerifyingKey::<Sha256>::from(public_key);
    Ok(Self { key })
  }

  pub fn from_pkcs1_der(bytes: &[u8]) -> crate::Result<Self> {
    let public_key =
      RsaPublicKey::from_pkcs1_der(bytes).map_err(crate::Error::invalid_verifying_key)?;
    let key = VerifyingKey::<Sha256>::from(public_key);
    Ok(Self { key })
  }

  pub fn from_pkcs1_pem(pem: &str) -> crate::Result<Self> {
    let public_key =
      RsaPublicKey::from_pkcs1_pem(pem).map_err(crate::Error::invalid_verifying_key)?;
    let key = VerifyingKey::<Sha256>::from(public_key);
    Ok(Self { key })
  }
}

impl SignatureVerifier for RsaPssVerifier {
  async fn verify(&self, _bundle: &Bundle, data: &[u8], signature: &str) -> crate::Result<bool> {
    let signature =
      Signature::try_from(signature.as_bytes()).map_err(|_| crate::Error::InvalidSignature)?;
    Ok(self.key.verify(data, &signature).is_ok())
  }
}
