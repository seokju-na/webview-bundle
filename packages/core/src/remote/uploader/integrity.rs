use crate::integrity::{Algorithm, Integrity};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SignArgs {
  pub integrity: String,
}

pub type Signer = dyn Fn(
    &SignArgs,
  ) -> Pin<
    Box<
      dyn Future<Output = Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>>
        + Send
        + 'static,
    >,
  > + Send
  + Sync;

#[non_exhaustive]
pub struct IntegrityMaker {
  pub algorithm: Algorithm,
  pub sign: Option<Arc<Signer>>,
}

impl IntegrityMaker {
  pub fn new(algorithm: Algorithm, sign: Option<Arc<Signer>>) -> Self {
    Self { algorithm, sign }
  }

  pub(crate) async fn make(&self, data: &[u8]) -> crate::Result<String> {
    let integrity = Integrity::compute(self.algorithm, data);
    if let Some(sign) = &self.sign {
      let args = SignArgs {
        integrity: integrity.serialize(),
      };
      let signature = sign(&args).await.map_err(crate::Error::unknown)?;
      return Ok(signature);
    }
    Ok(integrity.serialize())
  }
}
