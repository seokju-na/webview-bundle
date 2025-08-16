mod bundle;
mod checksum;
mod error;
mod header;
mod index;
mod reader;
mod version;
mod writer;

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub use error::Error;
