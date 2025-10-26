use crate::integrity::{Algorithm, Integrity};
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;

pub fn verify_integrity(data: &[u8], integrity: impl Into<String>) -> crate::Result<()> {
  let integrity = Integrity::from_str(&integrity.into())?;
  if !integrity.validate(data) {
    return Err(crate::Error::IntegrityVerifyFailed);
  }
  Ok(())
}

#[derive(Debug, Clone)]
pub struct VerifyArgs {
  pub original: String,
  pub signature: String,
}

pub type Verifier = dyn Fn(
    &VerifyArgs,
  ) -> Pin<
    Box<
      dyn Future<Output = Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>>>
        + Send
        + 'static,
    >,
  > + Send
  + Sync;

pub async fn verify_integrity_with_signature(
  data: &[u8],
  signature: impl Into<String>,
  algorithm: Algorithm,
  verifier: Arc<Verifier>,
) -> crate::Result<()> {
  let original = Integrity::compute(algorithm, data);
  let args = VerifyArgs {
    original: original.serialize(),
    signature: signature.into(),
  };
  if !verifier(&args).await.map_err(crate::Error::unknown)? {
    return Err(crate::Error::IntegrityVerifyFailed);
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use base64ct::{Base64, Encoding};
  use p256::ecdsa::signature::{Signer, Verifier};
  use p256::ecdsa::{Signature, SigningKey, VerifyingKey};

  #[test]
  fn verify() {
    let data = b"test";
    let integrity = Integrity::compute(Algorithm::Sha512, data);
    let verified = verify_integrity(data, integrity.serialize());
    assert!(verified.is_ok());
    let failed = verify_integrity(
      b"test2",
      Integrity::compute(Algorithm::Sha256, data).serialize(),
    );
    assert!(failed.is_err());
  }

  #[tokio::test]
  async fn verify_with_signature() {
    let data = Integrity::compute(Algorithm::Sha512, b"test").serialize();
    let signing_key = SigningKey::random(&mut rand_core::OsRng);
    let signature: Signature = signing_key.sign(data.as_bytes());
    let signature_base64 = Base64::encode_string(&signature.to_bytes());
    let public_key_bytes = VerifyingKey::from(&signing_key)
      .to_encoded_point(false)
      .to_bytes();
    let verified = verify_integrity_with_signature(
      b"test",
      &signature_base64,
      Algorithm::Sha512,
      Arc::new(move |args: &VerifyArgs| {
        let public_key_bytes = public_key_bytes.clone();
        let signature_str = args.signature.clone();
        let original_str = args.original.clone();
        Box::pin(async move {
          let verifying_key = VerifyingKey::from_encoded_point(
            &p256::EncodedPoint::from_bytes(&public_key_bytes).unwrap(),
          )
          .unwrap();
          let signature_base64 =
            Base64::decode_vec(&signature_str).expect("fail to decode signature base64");
          let signature =
            Signature::from_slice(&signature_base64).expect("fail to decode p256 signature");
          Ok(
            verifying_key
              .verify(original_str.as_bytes(), &signature)
              .is_ok(),
          )
        })
      }),
    )
    .await;
    assert!(verified.is_ok());
  }
}
