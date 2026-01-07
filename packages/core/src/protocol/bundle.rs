use crate::protocol::uri::{DefaultUriResolver, UriResolver};
use crate::source::BundleSource;
use async_trait::async_trait;
use http::{header, HeaderValue, Method, Request, Response, StatusCode};
use http_range::HttpRange;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

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
  async fn handle(&self, request: Request<Vec<u8>>) -> crate::Result<super::ProtocolResponse> {
    let name = self
      .uri_resolver
      .resolve_bundle(request.uri())
      .ok_or(crate::Error::BundleNotFound)?;
    let path = self.uri_resolver.resolve_path(request.uri());

    if !(request.method() == Method::GET || request.method() == Method::HEAD) {
      let response = Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Vec::new().into())?;
      return Ok(response);
    }

    let mut resp = Response::builder();
    let descriptor = self.source.load_descriptor(&name).await?;

    if let Some(entry) = descriptor.index().get_entry(&path) {
      if let Some(resp_headers) = resp.headers_mut() {
        resp_headers.clone_from(&entry.headers());
        // insert content-type header
        if let Ok(content_type) = HeaderValue::try_from(entry.content_type()) {
          resp_headers.insert(header::CONTENT_TYPE, content_type);
        }
        // insert content-length header
        let content_length = HeaderValue::from(entry.content_length());
        resp_headers.insert(header::CONTENT_LENGTH, content_length);
      }

      if request.method() == Method::HEAD {
        let response = resp.status(StatusCode::OK).body(Vec::new().into())?;
        return Ok(response);
      }

      if let Some(range_header) = request
        .headers()
        .get(header::RANGE)
        .and_then(|x| x.to_str().map(|x| x.to_string()).ok())
      {
        resp = resp.header(header::ACCEPT_RANGES, "bytes");
        resp = resp.header(header::ACCESS_CONTROL_EXPOSE_HEADERS, "content-range");

        let len = entry.content_length();
        let not_stisifiable = || {
          Response::builder()
            .status(StatusCode::RANGE_NOT_SATISFIABLE)
            .header(header::CONTENT_RANGE, format!("bytes */{len}"))
            .body(Vec::new().into())
            .map_err(Into::into)
        };

        let ranges = if let Ok(ranges) = HttpRange::parse(&range_header, len) {
          ranges
            .iter()
            // map the output to spec range <start-end>, example: 0-499
            .map(|x| (x.start, x.start + x.length - 1))
            .collect::<Vec<_>>()
        } else {
          return not_stisifiable();
        };

        /// The Maximum bytes we send in one range
        const MAX_LEN: u64 = 1000 * 1024;
        let adjust_end =
          |start: u64, end: u64, len: u64| start + (end - start).min(len - start).min(MAX_LEN - 1);

        // signle-part range header
        let response = if ranges.len() == 1 {
          let &(start, mut end) = ranges.first().unwrap();
          // check if a range is not satisfiable
          //
          // this should be already taken care of by the range parsing library
          // but checking here again for extra assurance
          if start >= len || end >= len || end < start {
            return not_stisifiable();
          }
          end = adjust_end(start, end, len);

          let reader = self.source.reader(&name).await?;
          let buf = if let Some(data) = descriptor.async_get_data(reader, &path).await? {
            extract_buf(&data, start, end)
          } else {
            return not_found();
          };

          resp = resp.header(header::CONTENT_RANGE, format!("bytes {start}-{end}/{len}"));
          resp = resp.header(header::CONTENT_LENGTH, end + 1 - start);
          resp = resp.status(StatusCode::PARTIAL_CONTENT);
          resp.body(buf.into())
        } else {
          let ranges = ranges
            .iter()
            .filter_map(|&(start, mut end)| {
              // filter out unsatisfiable ranges
              //
              // this should be already taken care of by the range parsing library
              // but checking here again for extra assurance
              if start >= len || end >= len || end < start {
                None
              } else {
                // adjust end byte for MAX_LEN
                end = adjust_end(start, end, len);
                Some((start, end))
              }
            })
            .collect::<Vec<_>>();

          let boundary = random_boundary();
          let boundary_sep = format!("\r\n--{boundary}\r\n");

          resp = resp.header(
            header::CONTENT_TYPE,
            format!("multipart/byteranges; boundary={boundary}"),
          );

          let reader = self.source.reader(&name).await?;
          let buf = if let Some(data) = descriptor.async_get_data(reader, &path).await? {
            let mut buf = Vec::new();
            for (start, end) in ranges {
              buf.write_all(boundary_sep.as_bytes()).await?;
              buf
                .write_all(
                  format!("{}: {}\r\n", header::CONTENT_TYPE, entry.content_type()).as_bytes(),
                )
                .await?;
              buf
                .write_all(
                  format!("{}: bytes {start}-{end}/{len}\r\n", header::CONTENT_RANGE).as_bytes(),
                )
                .await?;
              buf.write_all("\r\n".as_bytes()).await?;

              let range_buf = extract_buf(&data, start, end);
              buf.extend_from_slice(&range_buf);
            }
            buf.write_all(boundary_sep.as_bytes()).await?;
            buf
          } else {
            return not_found();
          };
          resp.body(buf.into())
        }?;

        return Ok(response);
      }

      let reader = self.source.reader(&name).await?;
      let data = if let Some(data) = descriptor.async_get_data(reader, &path).await? {
        data
      } else {
        return not_found();
      };

      let response = resp.body(data.into())?;
      Ok(response)
    } else {
      not_found()
    }
  }
}

fn not_found() -> crate::Result<super::ProtocolResponse> {
  let resp = Response::builder()
    .status(StatusCode::NOT_FOUND)
    .body(Vec::new().into())?;
  Ok(resp)
}

fn random_boundary() -> String {
  let mut values = [0_u8; 30];
  getrandom::fill(&mut values).expect("failed to get random bytes");
  (values[..])
    .iter()
    .map(|&val| format!("{val:x}"))
    .fold(String::new(), |mut acc, x| {
      acc.push_str(x.as_str());
      acc
    })
}

fn extract_buf(data: &[u8], start: u64, end: u64) -> Vec<u8> {
  let data_len = data.len() as u64;
  let start_i = start.min(data_len);
  let end_i = end.min(data_len.saturating_sub(1));

  let capacity = end + 1 - start;
  let mut buf = Vec::with_capacity(capacity as usize);
  if start_i <= end_i {
    let s = start as usize;
    let e = (end + 1) as usize;
    buf.extend_from_slice(&data[s..e]);
  }
  buf
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::protocol::Protocol;
  use crate::testing::Fixtures;

  #[tokio::test]
  async fn smoke() {
    let fixture = Fixtures::bundles();
    let source = Arc::new(
      BundleSource::builder()
        .builtin_dir(fixture.get_path("builtin"))
        .remote_dir(fixture.get_path("remote"))
        .build(),
    );
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://app.wvb/index.html")
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
          .uri("https://app.wvb/not_found.html")
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
            .uri("https://app.wvb/index.html")
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
    let fixture = Fixtures::bundles();
    let source = Arc::new(
      BundleSource::builder()
        .builtin_dir(fixture.get_path("builtin"))
        .remote_dir(fixture.get_path("remote"))
        .build(),
    );
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://app.wvb")
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
    let fixture = Fixtures::bundles();
    let source = Arc::new(
      BundleSource::builder()
        .builtin_dir(fixture.get_path("builtin"))
        .remote_dir(fixture.get_path("remote"))
        .build(),
    );
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://app.wvb/_next/static/chunks/framework-98177fb2e8834792.js")
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
          .uri("https://app.wvb/_next/static/css/fbfc89e8c66c1961.css")
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
          .uri("https://app.wvb/_next/static/media/build.583ad785.png")
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
    let fixture = Fixtures::bundles();
    let source = Arc::new(
      BundleSource::builder()
        .builtin_dir(fixture.get_path("builtin"))
        .remote_dir(fixture.get_path("remote"))
        .build(),
    );
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://app.wvb/_next/static/chunks/framework-98177fb2e8834792.js")
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
          .uri("https://app.wvb/_next/static/css/fbfc89e8c66c1961.css")
          .method("GET")
          .body(vec![])
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(
      resp.headers().get(header::CONTENT_LENGTH).unwrap(),
      HeaderValue::from_static("13926")
    );
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://app.wvb/_next/static/media/build.583ad785.png")
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
    let fixture = Fixtures::bundles();
    let source = Arc::new(
      BundleSource::builder()
        .builtin_dir(fixture.get_path("builtin"))
        .remote_dir(fixture.get_path("remote"))
        .build(),
    );
    let protocol = Arc::new(BundleProtocol::new(source.clone()));
    let resp = protocol
      .handle(
        Request::builder()
          .uri("https://app.wvb/path/does/not/exists")
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
    let fixture = Fixtures::bundles();
    let source = Arc::new(
      BundleSource::builder()
        .builtin_dir(fixture.get_path("builtin"))
        .remote_dir(fixture.get_path("remote"))
        .build(),
    );
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
