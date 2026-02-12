//! HTTP client for downloading bundles from a remote server.
//!
//! The remote module implements the client side of the bundle HTTP protocol,
//! allowing applications to:
//!
//! - List available bundles on a server
//! - Fetch bundle metadata (version, integrity, signature)
//! - Download specific bundle versions
//! - Verify bundle integrity before installation
//!
//! ## HTTP API Endpoints
//!
//! - `GET /bundles` - List all available bundles
//! - `HEAD /bundles/{name}` - Get bundle metadata without downloading
//! - `GET /bundles/{name}` - Download the current version of a bundle
//! - `GET /bundles/{name}/{version}` - Download a specific version
//!
//! ## Example
//!
//! ```no_run
//! # #[cfg(all(feature = "remote", feature = "source"))]
//! # async {
//! use wvb::remote::Remote;
//! use wvb::source::BundleSource;
//!
//! let remote = Remote::new("https://updates.example.com");
//! let source = BundleSource::builder()
//!     .remote_dir("./remote")
//!     .build();
//!
//! // List available bundles
//! let bundles = remote.list_bundles().await.unwrap();
//!
//! // Get bundle info
//! let info = remote.fetch_bundle("app").await.unwrap();
//! println!("Latest version: {}", info.version);
//!
//! // Download and install
//! remote.download_and_write(&source, "app", &info).await.unwrap();
//! # };
//! ```
//!
//! ## Headers
//!
//! Bundle metadata is communicated via HTTP headers:
//!
//! - `Webview-Bundle-Name`: Bundle identifier
//! - `Webview-Bundle-Version`: Version string
//! - `Webview-Bundle-Integrity`: Optional integrity hash for verification
//! - `Webview-Bundle-Signature`: Optional digital signature

mod http;
mod remote;

pub use http::*;
pub use remote::*;
