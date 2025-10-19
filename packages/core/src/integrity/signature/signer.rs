pub trait Signer {
  fn sign(&self, data: &[u8]) -> Vec<u8>;
}
