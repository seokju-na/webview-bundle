use crate::Bundle;

#[cfg(feature = "signature-ecdsa_secp256r1")]
mod ecdsa_secp256r1;
#[cfg(feature = "signature-edd25519")]
mod ed25519;

#[cfg(feature = "signature-ecdsa_secp256r1")]
pub use ecdsa_secp256r1::*;
#[cfg(feature = "signature-edd25519")]
pub use ed25519::*;

pub trait Signer: Send + Sync + 'static {
  fn sign(
    &self,
    bundle: &Bundle,
    data: &[u8],
  ) -> impl std::future::Future<Output = crate::Result<String>>;
}

pub trait Verifier: Send + Sync + 'static {
  fn verify(
    &self,
    signature: &str,
    bundle: &Bundle,
    data: &[u8],
  ) -> impl std::future::Future<Output = crate::Result<bool>>;
}
