use crate::mime_type::MimeType;
use crate::source::Source;
use crate::uri::{DefaultUriResolver, UriResolver};
use async_trait::async_trait;
use dashmap::DashMap;
use std::borrow::Cow;
use std::sync::Arc;
use tokio::sync::OnceCell;
use webview_bundle::http::{header, HeaderValue, Method, Request, Response};
use webview_bundle::BundleManifest;

pub struct BundleProtocol<S: Source> {
  source: Arc<S>,
  uri_resolver: Box<dyn UriResolver + 'static>,
  manifests: DashMap<String, Arc<OnceCell<Arc<BundleManifest>>>>,
}

impl<S: Source> BundleProtocol<S> {
  pub fn new(source: S) -> Self {
    Self {
      source: Arc::new(source),
      uri_resolver: Box::new(DefaultUriResolver),
      manifests: DashMap::default(),
    }
  }
}

impl<S: Source> BundleProtocol<S> {
  pub async fn load_manifest(&self, name: &str) -> crate::Result<Arc<BundleManifest>> {
    if let Some(entry) = self.manifests.get(name) {
      if let Some(m) = entry.get() {
        return Ok(m.clone());
      }
    }
    let cell_arc = {
      let entry = self.manifests.entry(name.to_string()).or_default();
      entry.clone()
    };
    let m = cell_arc
      .get_or_try_init(|| async {
        let m = self.source.fetch_manifest(name).await?;
        Ok::<Arc<BundleManifest>, crate::Error>(Arc::new(m))
      })
      .await?
      .clone();
    Ok(m)
  }

  pub fn unload_manifest(&self, name: &str) -> bool {
    self.manifests.remove(name).is_some()
  }
}

#[async_trait]
impl<S: Source> super::Protocol for BundleProtocol<S> {
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

    let manifest = self.load_manifest(&name).await?;
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
        headers_mut.insert(header::CONTENT_TYPE, HeaderValue::try_from(&mime)?);
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
  use crate::source::FileSource;
  use std::path::PathBuf;

  #[tokio::test]
  async fn load_many_at_once() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let protocol = Arc::new(BundleProtocol::new(source));
    let mut handles = Vec::new();
    for _i in 0..10 {
      let p = protocol.clone();
      let handle = tokio::spawn(async move {
        let _ = p.load_manifest("nextjs.wvb").await;
      });
      handles.push(handle);
    }
    for h in handles {
      h.await.unwrap();
    }
  }

  #[tokio::test]
  async fn load_unload_sequential() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let protocol = BundleProtocol::new(source);

    let m1 = protocol.load_manifest("nextjs.wvb").await.unwrap();
    assert!(
      protocol.unload_manifest("nextjs.wvb"),
      "unload should remove existing entry"
    );
    let m2 = protocol.load_manifest("nextjs.wvb").await.unwrap();
    assert!(
      !Arc::ptr_eq(&m1, &m2),
      "after unload, reloading should produce a new Arc"
    );

    assert!(protocol.unload_manifest("nextjs.wvb"));
    let m3 = protocol.load_manifest("nextjs.wvb").await.unwrap();
    assert!(!Arc::ptr_eq(&m2, &m3));

    assert!(protocol.unload_manifest("nextjs.wvb"));
    let m4 = protocol.load_manifest("nextjs.wvb").await.unwrap();
    assert!(!Arc::ptr_eq(&m3, &m4));
  }

  #[tokio::test]
  async fn load_unload_concurrently() {
    use std::sync::Arc;
    use tokio::sync::Barrier;
    use tokio::task::JoinSet;

    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let protocol = Arc::new(BundleProtocol::new(source));

    // 1) initial loads. test singleflight
    let n = 5usize;
    let mut set = JoinSet::new();
    for _i in 0..n {
      let p = protocol.clone();
      set.spawn(async move { p.load_manifest("nextjs.wvb").await });
    }
    let mut initials = Vec::with_capacity(n);
    while let Some(res) = set.join_next().await {
      let v = res.unwrap().unwrap();
      initials.push(v);
    }
    for m in &initials[1..] {
      assert!(Arc::ptr_eq(&initials[0], m));
    }

    // 2) before/after barriers
    let barrier_before_unload = Arc::new(Barrier::new(n + 1));
    let barrier_after_unload = Arc::new(Barrier::new(n + 1));

    let mut before_set = JoinSet::new();
    for _i in 0..n {
      let p = protocol.clone();
      let before = barrier_before_unload.clone();
      before_set.spawn(async move {
        before.wait().await;
        p.load_manifest("nextjs.wvb").await
      });
    }
    let mut after_set = JoinSet::new();
    for _i in 0..n {
      let p = protocol.clone();
      let after = barrier_after_unload.clone();
      after_set.spawn(async move {
        after.wait().await;
        p.load_manifest("nextjs.wvb").await
      });
    }

    barrier_before_unload.wait().await;
    assert!(protocol.unload_manifest("nextjs.wvb"));
    barrier_after_unload.wait().await;

    let mut before_jobs = Vec::with_capacity(n);
    while let Some(res) = before_set.join_next().await {
      let v = res.unwrap().unwrap();
      before_jobs.push(v);
    }
    let mut after_jobs = Vec::with_capacity(n);
    while let Some(res) = after_set.join_next().await {
      let v = res.unwrap().unwrap();
      after_jobs.push(v);
    }
    // before jobs should be same with initial loads
    for m in &before_jobs {
      assert!(Arc::ptr_eq(&initials[0], m));
    }
    // after jobs should be not same with initial loads
    for m in &after_jobs {
      assert!(!Arc::ptr_eq(&initials[0], m));
    }
    for m in &before_jobs[1..] {
      assert!(Arc::ptr_eq(&before_jobs[0], m));
    }
    for m in &after_jobs[1..] {
      assert!(Arc::ptr_eq(&after_jobs[0], m));
    }
  }

  #[tokio::test]
  async fn smoke() {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let protocol = Arc::new(BundleProtocol::new(source));
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
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let protocol = Arc::new(BundleProtocol::new(source));
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
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let protocol = Arc::new(BundleProtocol::new(source));
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
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let protocol = Arc::new(BundleProtocol::new(source));
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
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let protocol = Arc::new(BundleProtocol::new(source));
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
      .join("fixtures");
    let source = FileSource::new(base_dir);
    let protocol = Arc::new(BundleProtocol::new(source));
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
