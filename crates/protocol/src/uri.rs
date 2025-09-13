use webview_bundle::http::Uri;

pub trait UriResolver: Send + Sync {
  fn resolve_bundle(&self, uri: &Uri) -> Option<String>;
  fn resolve_path(&self, uri: &Uri) -> String;
}

#[derive(Default)]
pub struct DefaultUriResolver;

impl UriResolver for DefaultUriResolver {
  fn resolve_bundle(&self, uri: &Uri) -> Option<String> {
    uri
      .host()
      .and_then(|x| x.split('.').next())
      .map(|x| x.to_string())
  }

  fn resolve_path(&self, uri: &Uri) -> String {
    let mut path = uri.path().to_string();
    if path.ends_with('/') {
      path.push_str("index.html");
      return path;
    }
    if let Some(last) = path.rsplit('/').next() {
      if !last.is_empty() && !last.contains('.') {
        path.push_str("/index.html");
      }
    }
    path
  }
}
