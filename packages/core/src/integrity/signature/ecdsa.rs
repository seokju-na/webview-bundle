#[cfg(feature = "integrity-signature-signing")]
pub mod signing {
  use crate::integrity::signature;
  use ecdsa::elliptic_curve::CurveArithmetic;
  use ecdsa::signature::Signer;
  use ecdsa::{elliptic_curve, EcdsaCurve, SigningKey, ECDSA_SHA256_OID};

  #[derive(Debug)]
  pub struct EcdsaSigner {
    // curve:
    // raw: Vec<u8>,
  }

  impl signature::Signer for EcdsaSigner {
    fn sign(&self, data: &[u8]) -> crate::Result<Vec<u8>> {
      let key = SigningKey::<elliptic_curve::Curve>::from_slice(&self.key_raw).unwrap();
      key.sign(data).to_vec()
    }
  }
}
