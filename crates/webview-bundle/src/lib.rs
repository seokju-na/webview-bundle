mod builder;
mod bundle;
mod decoder;
mod encoder;
mod error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub use bundle::{Bundle, Version};
pub use decoder::decode;
pub use encoder::encode;
pub use error::Error;
