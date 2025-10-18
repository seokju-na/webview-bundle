pub trait Signer {
  fn sign(&self, data: &[u8]) -> crate::Result<Vec<u8>>;
}
