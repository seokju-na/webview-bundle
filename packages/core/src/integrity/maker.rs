use crate::integrity::{Integrity, IntegrityAlgorithm};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type CustomMaker = dyn Fn(
    &[u8],
  ) -> Pin<
    Box<
      dyn Future<Output = Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>>
        + Send
        + 'static,
    >,
  > + Send
  + Sync;

#[non_exhaustive]
pub enum IntegrityMaker {
  Default(Option<IntegrityAlgorithm>),
  Custom(Arc<CustomMaker>),
}

impl Default for IntegrityMaker {
  fn default() -> Self {
    Self::Default(None)
  }
}

impl IntegrityMaker {
  pub async fn make(&self, data: &[u8]) -> crate::Result<String> {
    match self {
      Self::Default(alg) => {
        let alg = alg.unwrap_or_default();
        let integrity = Integrity::compute(alg, data);
        Ok(integrity.serialize())
      }
      Self::Custom(maker) => {
        let integrity = maker(data).await.map_err(crate::Error::unknown)?;
        Ok(integrity)
      }
    }
  }
}
