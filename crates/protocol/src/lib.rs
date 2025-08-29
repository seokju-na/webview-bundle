mod error;
mod fetcher;
mod loader;
mod protocol;
mod uri;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
