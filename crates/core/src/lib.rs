mod builder;
mod bundle;
mod decoder;
mod encoder;
mod error;
mod index;
mod reader;
mod version;

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub use builder::Builder;
pub use bundle::{Bundle, BundleFile, Version};
pub use decoder::decode;
pub use encoder::{encode, encode_bytes};
pub use error::Error;
