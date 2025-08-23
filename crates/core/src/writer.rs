pub trait Writer<T> {
  fn write(&mut self, data: &T) -> crate::Result<usize>;
}
