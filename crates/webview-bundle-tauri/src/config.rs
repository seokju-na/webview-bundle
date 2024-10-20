use crate::loader::Loader;

pub struct Config<L: Loader + Send + Sync> {
  loader: L,
}

#[buildstructor::buildstructor]
impl<L> Config<L>
where
  L: Loader + Send + Sync,
{
  #[builder]
  pub fn new(loader: L) -> Self {
    Self { loader }
  }

  pub fn loader(&self) -> &L {
    &self.loader
  }
}
