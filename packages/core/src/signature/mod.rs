use crate::Bundle;
use std::pin::Pin;
use std::sync::Arc;

#[cfg(feature = "signature-ecdsa_secp256r1")]
mod ecdsa_secp256r1;
#[cfg(feature = "signature-ecdsa_secp384r1")]
mod ecdsa_secp384r1;
#[cfg(feature = "signature-edd25519")]
mod ed25519;
#[cfg(feature = "signature-rsa_pkcs1_v1_5")]
mod rsa_pkcs1_v1_5;
#[cfg(feature = "signature-rsa_pss")]
mod rsa_pss;
#[cfg(feature = "signature-ecdsa_secp256r1")]
pub use ecdsa_secp256r1::*;
#[cfg(feature = "signature-ecdsa_secp384r1")]
pub use ecdsa_secp384r1::*;
#[cfg(feature = "signature-edd25519")]
pub use ed25519::*;
#[cfg(feature = "signature-rsa_pkcs1_v1_5")]
pub use rsa_pkcs1_v1_5::*;
#[cfg(feature = "signature-rsa_pss")]
pub use rsa_pss::*;

pub type CustomVerify = dyn Fn(
    &Bundle,
    &[u8],
    &str,
  ) -> Pin<
    Box<
      dyn std::future::Future<
          Output = Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>>,
        > + Send
        + 'static,
    >,
  > + Send
  + Sync;

#[non_exhaustive]
pub enum SignatureVerifier {
  #[cfg(feature = "signature-ecdsa_secp256r1")]
  EcdsaSecp256r1(Arc<EcdsaSecp256r1Verifier>),
  #[cfg(feature = "signature-ecdsa_secp384r1")]
  EcdsaSecp384r1(Arc<EcdsaSecp384r1Verifier>),
  #[cfg(feature = "signature-edd25519")]
  Ed25519(Arc<Ed25519Verifier>),
  #[cfg(feature = "signature-rsa_pkcs1_v1_5")]
  RsaPkcs1V15(Arc<RsaPkcs1V15Verifier>),
  #[cfg(feature = "signature-rsa_pss")]
  RsaPss(Arc<RsaPssVerifier>),
  Custom(Arc<CustomVerify>),
}

impl SignatureVerifier {
  pub async fn verify(
    &self,
    bundle: &Bundle,
    message: &[u8],
    signature: &str,
  ) -> crate::Result<bool> {
    match self {
      Self::Custom(verify) => verify(bundle, message, signature)
        .await
        .map_err(crate::Error::generic),
      #[cfg(feature = "signature-ecdsa_secp256r1")]
      Self::EcdsaSecp256r1(verifier) => verifier.verify(bundle, message, signature).await,
      #[cfg(feature = "signature-ecdsa_secp384r1")]
      Self::EcdsaSecp384r1(verifier) => verifier.verify(bundle, message, signature).await,
      #[cfg(feature = "signature-edd25519")]
      Self::Ed25519(verifier) => verifier.verify(bundle, message, signature).await,
      #[cfg(feature = "signature-rsa_pkcs1_v1_5")]
      Self::RsaPkcs1V15(verifier) => verifier.verify(bundle, message, signature).await,
      #[cfg(feature = "signature-rsa_pss")]
      Self::RsaPss(verifier) => verifier.verify(bundle, message, signature).await,
    }
  }
}

pub trait Verifier: Send + Sync + 'static {
  fn verify(
    &self,
    bundle: &Bundle,
    message: &[u8],
    signature: &str,
  ) -> impl std::future::Future<Output = crate::Result<bool>>;
}
