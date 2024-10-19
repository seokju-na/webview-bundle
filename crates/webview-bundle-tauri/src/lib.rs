use tauri::http::{Method, Response};
use tauri::plugin::TauriPlugin;
use tauri::{plugin, Runtime};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  plugin::Builder::<R>::new("webview-bundle")
    .register_asynchronous_uri_scheme_protocol("app", |_ctx, request, responder| {
      let method = request.method();
      if method != Method::GET {
        responder.respond(Response::builder().status(405).body(vec![]).unwrap());
        return;
      }
      tokio::spawn(async move {
        responder.respond(
          Response::builder()
            .header("content-type", "text/html")
            .header("content-length", 0)
            .status(200)
            .body(vec![])
            .unwrap(),
        );
      });
    })
    .build()
}
