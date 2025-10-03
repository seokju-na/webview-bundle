pub trait Reader<T> {
  fn read(&mut self) -> crate::Result<T>;
}

#[cfg(feature = "async")]
pub trait AsyncReader<T> {
  fn read(&mut self) -> impl std::future::Future<Output = crate::Result<T>>;
}
