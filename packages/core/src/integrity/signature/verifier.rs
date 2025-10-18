pub trait Verifier {
  fn verify(&self, data: &[u8], signature: &[u8]) -> crate::Result<()>;
}
