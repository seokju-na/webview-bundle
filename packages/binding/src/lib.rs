#![cfg(any(target_os = "macos", target_os = "linux", windows))]
#![allow(clippy::new_without_default)]

use napi::bindgen_prelude::*;
use napi::{JsBuffer, JsBufferValue, Ref};
use napi_derive::napi;

#[napi(js_name = "Bundle")]
pub struct JsBundle {
  inner: webview_bundle::Bundle,
}

#[napi]
impl JsBundle {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      inner: webview_bundle::Bundle::builder().build(),
    }
  }

  #[napi]
  pub fn encode(&self) -> Result<Buffer> {
    let encoded = webview_bundle::encode_bytes(&self.inner)
      .map_err(|e| Error::new(Status::GenericFailure, e))?;
    Ok(Buffer::from(encoded))
  }
}

pub struct Parser {
  data: Ref<JsBufferValue>,
}

#[napi]
impl Task for Parser {
  type Output = webview_bundle::Bundle;
  type JsValue = JsBundle;

  fn compute(&mut self) -> Result<Self::Output> {
    if self.data.len() == 0 {
      return Err(Error::new(Status::InvalidArg, "empty buffer"));
    }
    match webview_bundle::decode(self.data.as_ref()) {
      Ok(bundle) => Ok(bundle),
      Err(e) => Err(Error::new(Status::GenericFailure, e)),
    }
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(JsBundle { inner: output })
  }

  fn finally(&mut self, env: Env) -> Result<()> {
    self.data.unref(env)?;
    Ok(())
  }
}

#[napi]
pub fn parse(buf: JsBuffer) -> Result<AsyncTask<Parser>> {
  let data = buf.into_ref()?;
  Ok(AsyncTask::new(Parser { data }))
}
