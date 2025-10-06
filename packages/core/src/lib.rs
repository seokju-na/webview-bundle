mod builder;
mod bundle;
mod checksum;
mod error;
mod header;
mod index;
mod reader;
mod version;
mod writer;

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub use builder::*;
pub use bundle::*;
pub use consts::*;
pub use error::Error;
pub use header::*;
pub use index::*;
pub use reader::*;
pub use version::*;
pub use writer::*;

pub use http;

mod consts;
#[cfg(feature = "protocol")]
pub mod protocol;
#[cfg(feature = "remote")]
pub mod remote;
#[cfg(feature = "source")]
pub mod source;
#[cfg(feature = "updater")]
pub mod updater;
