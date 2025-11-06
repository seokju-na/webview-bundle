use crate::integrity::Integrity;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;

pub type CustomIntegrityChecker = dyn Fn(
    &str,
    &[u8],
  ) -> Pin<
    Box<
      dyn Future<Output = Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>>>
        + Send
        + 'static,
    >,
  > + Send
  + Sync;

#[non_exhaustive]
pub enum IntegrityChecker {
  Default,
  Custom(Arc<CustomIntegrityChecker>),
}

impl Default for IntegrityChecker {
  fn default() -> Self {
    Self::Default
  }
}

impl IntegrityChecker {
  pub async fn check(&self, integrity: &str, data: &[u8]) -> crate::Result<()> {
    match self {
      Self::Default => {
        let integrity = Integrity::from_str(integrity)?;
        if !integrity.validate(data) {
          return Err(crate::Error::IntegrityVerifyFailed);
        }
        Ok(())
      }
      Self::Custom(checker) => {
        if !checker(integrity, data)
          .await
          .map_err(crate::Error::unknown)?
        {
          return Err(crate::Error::IntegrityVerifyFailed);
        }
        Ok(())
      }
    }
  }
}
