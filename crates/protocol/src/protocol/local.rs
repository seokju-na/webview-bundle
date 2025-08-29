use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use webview_bundle::http;

#[derive(Clone)]
struct CachedResponse {
  status: http::StatusCode,
  headers: http::HeaderMap,
  body: bytes::Bytes,
}

pub struct LocalProtocol {
  cache: Arc<Mutex<HashMap<String, CachedResponse>>>,
}

impl super::protocol::Protocol for LocalProtocol {
  fn handle(
    &self,
    request: http::Request<Vec<u8>>,
  ) -> crate::Result<http::Response<Cow<'static, [u8]>>> {
    let url = "http://localhost:3000";

    let mut builder = http::Response::builder().header("Access-Control-Allow-Origin", "*");

    let client = reqwest::ClientBuilder::new();
    let mut proxy_builder = client.build()?.request(request.method().clone(), url);
    proxy_builder = proxy_builder.body(request.body().clone());
    let r = block_on(proxy_builder.send())??;
    let mut cache = self.cache.lock().unwrap();
    let mut response = None;
    if r.status() == http::StatusCode::NOT_MODIFIED {
      response = cache.get(url);
    }
    let response = if let Some(r) = response {
      r
    } else {
      let status = r.status();
      let headers = r.headers().clone();
      let body = block_on(r.bytes())??;
      let response = CachedResponse {
        status,
        headers,
        body,
      };
      cache.insert(url.to_string(), response);
      cache.get(url).unwrap()
    };
    for (name, value) in &response.headers {
      builder = builder.header(name, value);
    }
    builder
      .status(response.status)
      .body(response.body.to_vec().into())
      .map_err(|e| crate::Error::Core(e.into()))
  }
}

fn block_on<F>(task: F) -> crate::Result<F::Output>
where
  F: Future + Send + 'static,
  F::Output: Send + 'static,
{
  let handle = tokio::runtime::Handle::try_current()?;
  let (tx, rx) = std::sync::mpsc::sync_channel(1);
  let cloned = handle.clone();
  handle.spawn_blocking(move || {
    tx.send(cloned.block_on(task)).unwrap();
  });
  Ok(rx.recv().unwrap())
}
