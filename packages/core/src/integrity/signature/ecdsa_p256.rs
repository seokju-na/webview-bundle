use crate::integrity::signature;
use p256::ecdsa::signature::{Signer, Verifier};
use p256::ecdsa::{Signature, SigningKey, VerifyingKey};

#[derive(Debug)]
pub struct EcdsaP256Signer {
  key: SigningKey,
}

impl signature::Signer for EcdsaP256Signer {
  fn sign(&self, data: &[u8]) -> Vec<u8> {
    let signature: Signature = self.key.sign(data);
    signature.to_vec()
  }
}

#[derive(Debug)]
pub struct EcdsaP256Verifier {
  key: VerifyingKey,
}

impl signature::Verifier for EcdsaP256Verifier {
  fn verify(&self, data: &[u8], signature: &[u8]) -> crate::Result<()> {
    let signature = Signature::from_slice(signature).map_err(|_| crate::Error::InvalidSignature)?;
    self
      .key
      .verify(data, &signature)
      .map_err(|_| crate::Error::VerifyFailed)?;
    Ok(())
  }
}
