use std::fmt::Write;
use std::panic::{set_hook, PanicHookInfo};
use std::thread;

/// Installs a global panic handler to show a user-friendly error message in case the CLI panics
pub fn setup_panic_handler() {
  set_hook(Box::new(panic_handler))
}

fn panic_handler(info: &PanicHookInfo) {
  // Buffer the error message to a string before printing it at once
  // to prevent it from getting mixed with other errors if multiple threads
  // panic at the same time
  let mut error = String::new();

  writeln!(error, "An unexpected error").unwrap();
  writeln!(error).unwrap();

  writeln!(error, "This is a bug, not an error in your code, and we would appreciate it if you could report it to https://github.com/seokju-na/webview-bundle/issues/ along with the following information to help us fixing the issue:").unwrap();
  writeln!(error).unwrap();

  if let Some(location) = info.location() {
    writeln!(error, "Source Location: {location}").unwrap();
  }

  if let Some(thread) = thread::current().name() {
    writeln!(error, "Thread Name: {thread}").unwrap();
  }

  let payload = info.payload();
  if let Some(msg) = payload.downcast_ref::<&'static str>() {
    writeln!(error, "Message: {msg}").unwrap();
  } else if let Some(msg) = payload.downcast_ref::<String>() {
    writeln!(error, "Message: {msg}").unwrap();
  }

  eprintln!("{error}");
  tracing::error!("{error}");
}
