#![cfg(any(target_os = "macos", target_os = "linux", windows))]
#![allow(clippy::new_without_default)]

use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct BundleFile {
  pub path: String,
  pub data: Buffer,
}

impl From<webview_bundle::BundleFile> for BundleFile {
  fn from(value: webview_bundle::BundleFile) -> Self {
    Self {
      path: value.path,
      data: Buffer::from(value.data),
    }
  }
}

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
  pub async fn read_all_files(&self) -> Result<Vec<BundleFile>> {
    let files = self
      .inner
      .read_all_files()
      .map_err(|e| Error::new(Status::GenericFailure, e))?
      .into_iter()
      .map(BundleFile::from)
      .collect::<Vec<_>>();
    Ok(files)
  }

  #[napi]
  pub async fn read_file(&self, path: String) -> Result<BundleFile> {
    let file = self
      .inner
      .read_file(&path)
      .map_err(|e| Error::new(Status::GenericFailure, e))?;
    Ok(BundleFile::from(file))
  }

  #[napi]
  pub async fn read_file_data(&self, path: String) -> Result<Buffer> {
    let file = self.read_file(path).await?;
    Ok(file.data)
  }
}

#[napi(ts_args_type = "buf: Buffer, callback: (err: null | Error, result: Bundle) => void")]
pub fn decode(buf: Buffer, callback: JsFunction) -> Result<()> {
  let js_fn: ThreadsafeFunction<webview_bundle::Bundle, ErrorStrategy::CalleeHandled> =
    callback.create_threadsafe_function(0, |ctx| Ok(vec![JsBundle { inner: ctx.value }]))?;
  let data: Vec<u8> = buf.into();
  std::thread::spawn(move || match webview_bundle::decode(data) {
    Ok(bundle) => {
      js_fn.call(Ok(bundle), ThreadsafeFunctionCallMode::Blocking);
    }
    Err(e) => {
      js_fn.call(
        Err(Error::new(Status::GenericFailure, e)),
        ThreadsafeFunctionCallMode::Blocking,
      );
    }
  });
  Ok(())
}

#[napi(ts_args_type = "bundle: Bundle, callback: (err: null | Error, result: Buffer) => void")]
pub fn encode(bundle: &JsBundle, callback: JsFunction) -> Result<()> {
  let js_fn: ThreadsafeFunction<Vec<u8>, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx| {
      ctx
        .env
        .create_buffer_with_data(ctx.value)
        .map(|x| vec![x.into_raw()])
    })?;
  let b = bundle.inner.clone();
  std::thread::spawn(move || {
    match webview_bundle::encode_bytes(&b) {
      Ok(x) => {
        js_fn.call(Ok(x), ThreadsafeFunctionCallMode::Blocking);
      }
      Err(e) => {
        js_fn.call(
          Err(Error::new(Status::GenericFailure, e)),
          ThreadsafeFunctionCallMode::Blocking,
        );
      }
    };
  });
  Ok(())
}

pub struct BundleBuilder {
  files: Vec<BundleFile>,
}

impl Task for BundleBuilder {
  type Output = webview_bundle::Bundle;
  type JsValue = JsBundle;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut builder = webview_bundle::Bundle::builder();
    for file in self.files.iter() {
      let data: Vec<u8> = file.data.clone().into();
      builder = builder.add_file(&file.path, &data);
    }
    Ok(builder.build())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(JsBundle { inner: output })
  }
}

#[napi]
pub fn create(files: Vec<BundleFile>) -> AsyncTask<BundleBuilder> {
  AsyncTask::new(BundleBuilder { files })
}
