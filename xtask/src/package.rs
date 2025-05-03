use crate::Error;
use crate::actions::Actions;
use crate::config::{Config, PackageConfig};
use crate::version::{BumpRule, Version};
use crate::versioned_file::VersionedFile;
use crate::versioned_git_tag::VersionedGitTag;
use nonempty::NonEmpty;
use std::path::Path;

pub struct Package {
  name: String,
  versioned_files: Vec<VersionedFile>,
  config: PackageConfig,
}

impl Package {
  pub fn load_all(root_dir: &Path, config: &Config) -> Result<Vec<Self>, Error> {
    let mut packages = vec![];
    for (name, pkg) in config.packages.iter() {
      let versioned_files = VersionedFile::load_all(root_dir, &pkg.versioned_files)?;
      let versioned_files =
        NonEmpty::from_vec(versioned_files).ok_or(Error::InvalidPackageConfig {
          name: name.to_owned(),
          message: "versioned_files must not be empty".to_owned(),
        })?;
      let package = Self {
        name: name.to_string(),
        versioned_files: versioned_files.into(),
        config: pkg.clone(),
      };
      packages.push(package);
    }
    Ok(packages)
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn config(&self) -> &PackageConfig {
    &self.config
  }

  pub fn version(&self) -> &Version {
    self.versioned_files.first().unwrap().version()
  }

  pub fn next_version(&self) -> &Version {
    self.versioned_files.first().unwrap().next_version()
  }

  pub fn has_changed(&self) -> bool {
    self.versioned_files.first().unwrap().has_changed()
  }

  pub fn versioned_files(&self) -> &[VersionedFile] {
    &self.versioned_files
  }

  pub fn versioned_git_tag(&self) -> VersionedGitTag {
    VersionedGitTag::new(self.name(), self.version().clone())
  }

  pub fn next_versioned_git_tag(&self) -> VersionedGitTag {
    VersionedGitTag::new(self.name(), self.next_version().clone())
  }

  pub fn bump_version(&mut self, rule: &BumpRule) -> Result<(), Error> {
    for versioned_file in &mut self.versioned_files {
      versioned_file.bump_version(rule)?;
    }
    Ok(())
  }

  pub fn write(&self) -> Result<Vec<Actions>, Error> {
    let mut actions = vec![];
    for versioned_file in &self.versioned_files {
      actions.extend(versioned_file.write()?);
    }
    Ok(actions)
  }

  pub fn publish(&self) -> Result<Vec<Actions>, Error> {
    let mut actions = vec![];
    for versioned_file in &self.versioned_files {
      actions.extend(versioned_file.publish()?);
    }
    Ok(actions)
  }
}
