use crate::protocol::mime_type::MimeType;
use crate::protocol::uri::{DefaultUriResolver, UriResolver};
use crate::source::BundleSource;
use async_trait::async_trait;
use http::{header, HeaderValue, Method, Request, Response};
use std::borrow::Cow;
use std::sync::Arc;

pub struct BundleProtocol {
  source: Arc<BundleSource>,
  uri_resolver: Box<dyn UriResolver + 'static>,
}

impl BundleProtocol {
  pub fn new(source: Arc<BundleSource>) -> Self {
    Self {
      source,
      uri_resolver: Box::new(DefaultUriResolver),
    }
  }
}

#[async_trait]
impl super::Protocol for BundleProtocol {
  async fn handle(&self, request: Request<Vec<u8>>) -> crate::Result<Response<Cow<'static, [u8]>>> {
    let name = self
      .uri_resolver
      .resolve_bundle(request.uri())
      .ok_or(crate::Error::BundleNotFound)?;
    let path = self.uri_resolver.resolve_path(request.uri());

    let mut resp = Response::builder();

    if !(request.method() == Method::GET || request.method() == Method::HEAD) {
      let method_not_allowed = resp.status(405).body(Vec::new().into())?;
      return Ok(method_not_allowed);
    }

    let manifest = self.source.load_manifest(&name).await?;
    if !manifest.index().contains_path(&path) {
      let not_found = resp.status(404).body(Vec::new().into())?;
      return Ok(not_found);
    }

    let reader = self.source.reader(&name).await?;
    let data = manifest.async_get_data(reader, &path).await?.unwrap();

    if let Some(headers_mut) = resp.headers_mut() {
      if let Some(headers) = manifest.index().get_entry(&path).map(|x| x.headers()) {
        headers_mut.clone_from(headers);
      }
      // append if content-type header does not exists
      if !headers_mut.contains_key("content-type") {
        let mime = MimeType::parse(&data, &path);
        headers_mut.insert(
          header::CONTENT_TYPE,
          HeaderValue::try_from(&mime).expect("fail to convert mime type into header value"),
        );
      }
      // insert content-length header
      headers_mut.insert(header::CONTENT_LENGTH, data.len().into());
    }

    let response = resp.status(200).body(data.into())?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::protocol::Protocol;
  use std::path::PathBuf;

  #[tokio::test]
  async fn smoke() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = Arc::new(BundleSource::new(
      &base_dir.join("builtin"),
      &base_dir.join("remote"),
    ));
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://nextjs.wvb/index.html")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(resp.status(), 200);
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://nextjs.wvb/not_found.html")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(resp.status(), 404);
    let mut handlers = vec![];
    for _ in 1..100 {
      let p = protocol.clone();
      let handle = tokio::spawn(async move {
        p.handle(
          Request::builder()
            .uri("https://nextjs.wvb/index.html")
            .method("GET")
            .body(vec![])
            .unwrap(),
        )
        .await
      });
      handlers.push(handle);
    }
    for h in handlers {
      let resp = h.await.unwrap().unwrap();
      assert_eq!(resp.status(), 200);
    }
  }

  #[tokio::test]
  async fn resolve_index_html() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = Arc::new(BundleSource::new(
      &base_dir.join("builtin"),
      &base_dir.join("remote"),
    ));
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://nextjs.wvb")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(resp.status(), 200);
  }

  #[tokio::test]
  async fn content_type() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = Arc::new(BundleSource::new(
      &base_dir.join("builtin"),
      &base_dir.join("remote"),
    ));
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://nextjs.wvb/_next/static/chunks/framework-98177fb2e8834792.js")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(
      resp.headers().get(header::CONTENT_TYPE).unwrap(),
      HeaderValue::from_static("text/javascript")
    );
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://nextjs.wvb/_next/static/css/419406682a95b9a9.css")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(
      resp.headers().get(header::CONTENT_TYPE).unwrap(),
      HeaderValue::from_static("text/css")
    );
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://nextjs.wvb/_next/static/media/build.583ad785.png")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(
      resp.headers().get(header::CONTENT_TYPE).unwrap(),
      HeaderValue::from_static("image/png")
    );
  }

  #[tokio::test]
  async fn content_length() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = Arc::new(BundleSource::new(
      &base_dir.join("builtin"),
      &base_dir.join("remote"),
    ));
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://nextjs.wvb/_next/static/chunks/framework-98177fb2e8834792.js")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(
      resp.headers().get(header::CONTENT_LENGTH).unwrap(),
      HeaderValue::from_static("139833")
    );
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://nextjs.wvb/_next/static/css/419406682a95b9a9.css")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(
      resp.headers().get(header::CONTENT_LENGTH).unwrap(),
      HeaderValue::from_static("13856")
    );
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://nextjs.wvb/_next/static/media/build.583ad785.png")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(
      resp.headers().get(header::CONTENT_LENGTH).unwrap(),
      HeaderValue::from_static("475918")
    );
  }

  #[tokio::test]
  async fn not_found() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = Arc::new(BundleSource::new(
      &base_dir.join("builtin"),
      &base_dir.join("remote"),
    ));
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://nextjs.wvb/path/not/exists")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(resp.status(), 404);
  }

  #[tokio::test]
  async fn bundle_not_found() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures")
      .join("bundles");
    let source = Arc::new(BundleSource::new(
      &base_dir.join("builtin"),
      &base_dir.join("remote"),
    ));
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let err = protocol
      .handle(
        Request::builder()
          .uri("https://not_exsits_bundle.wvb")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap_err();
    assert!(matches!(err, crate::Error::BundleNotFound));
  }
}
