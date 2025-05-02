use crate::Error;
use crate::actions::Actions;
use crate::version::{BumpRule, Version};
use crate::versioned_file::cargo::Cargo;
use crate::versioned_file::package_json::PackageJson;
use crate::versioned_file::package_manager::PackageManager;
use glob::glob;
use relative_path::{PathExt, RelativePathBuf};
use std::path::Path;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum VersionedFileKind {
  Cargo,
  PackageJson,
}

pub struct VersionedFile {
  kind: VersionedFileKind,
  next_version: Option<Version>,
  pkg_manager: Box<dyn PackageManager>,
}

impl VersionedFile {
  pub fn load_all(root_dir: &Path, patterns: &[String]) -> Result<Vec<VersionedFile>, Error> {
    let mut versioned_files = vec![];
    for pattern in patterns {
      let fullpath = root_dir.join(pattern);
      let pattern = fullpath.to_str().unwrap();
      for entry in glob(pattern)? {
        let filepath = entry?;
        let relative_path = filepath.relative_to(root_dir).unwrap();
        let versioned_file = Self::load(root_dir, relative_path)?;
        versioned_files.push(versioned_file);
      }
    }
    Ok(versioned_files)
  }

  pub fn load(root_dir: &Path, path: RelativePathBuf) -> Result<Self, Error> {
    let filename = path.file_name();
    let filepath = path.to_path(root_dir);
    let content = std::fs::read_to_string(&filepath)?;
    match filename {
      Some("package.json") => {
        let versioned_file = PackageJson::new(path, content)?;
        Ok(Self {
          kind: VersionedFileKind::PackageJson,
          next_version: None,
          pkg_manager: Box::new(versioned_file),
        })
      }
      Some("Cargo.toml") => {
        let versioned_file = Cargo::new(path, content)?;
        Ok(Self {
          kind: VersionedFileKind::Cargo,
          next_version: None,
          pkg_manager: Box::new(versioned_file),
        })
      }
      _ => Err(Error::InvalidVersionedFile { path }),
    }
  }

  pub fn kind(&self) -> &VersionedFileKind {
    &self.kind
  }

  pub fn name(&self) -> &str {
    self.pkg_manager.name()
  }

  pub fn path(&self) -> &RelativePathBuf {
    self.pkg_manager.path()
  }

  pub fn version(&self) -> &Version {
    self.pkg_manager.version()
  }

  pub fn next_version(&self) -> &Version {
    self.next_version.as_ref().unwrap_or(self.version())
  }

  pub fn has_changed(&self) -> bool {
    self.version() != self.next_version()
  }

  pub fn bump_version(&mut self, rule: &BumpRule) -> Result<(), Error> {
    let mut next_version = self.version().clone();
    next_version.bump(rule)?;
    self.next_version = Some(next_version);
    Ok(())
  }

  pub fn write(&self) -> Result<Vec<Actions>, Error> {
    if !self.has_changed() {
      return Ok(vec![]);
    }
    self.pkg_manager.write(self.next_version())
  }

  pub fn publish(&self) -> Result<Vec<Actions>, Error> {
    if !self.has_changed() || !self.pkg_manager.can_publish() {
      return Ok(vec![]);
    }
    self.pkg_manager.publish(self.next_version())
  }
}
