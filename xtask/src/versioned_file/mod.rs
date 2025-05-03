mod cargo;
mod package_json;
mod package_manager;
#[allow(clippy::module_inception)]
mod versioned_file;

pub use versioned_file::{VersionedFile, VersionedFileKind};
