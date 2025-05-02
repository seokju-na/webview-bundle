use crate::Error;
use crate::actions::Actions;
use crate::cargo::{cargo_pkg_name, cargo_pkg_version, write_cargo_version};
use crate::version::Version;
use crate::versioned_file::package_manager::PackageManager;
use relative_path::RelativePathBuf;
use std::str::FromStr;
use toml_edit::DocumentMut;

pub struct Cargo {
  path: RelativePathBuf,
  doc: DocumentMut,
  name: String,
  version: Version,
}

impl Cargo {
  pub fn new(path: RelativePathBuf, content: String) -> Result<Self, Error> {
    let doc = DocumentMut::from_str(&content)?;
    let name = cargo_pkg_name(&doc)
      .ok_or(Error::InvalidVersionedFile {
        path: path.to_owned(),
      })?
      .to_string();
    let version = cargo_pkg_version(&doc)
      .and_then(|x| Version::parse(x).ok())
      .ok_or(Error::InvalidVersionedFile {
        path: path.to_owned(),
      })?;

    Ok(Self {
      path,
      doc,
      name,
      version,
    })
  }
}

impl PackageManager for Cargo {
  fn name(&self) -> &str {
    &self.name
  }

  fn path(&self) -> &RelativePathBuf {
    &self.path
  }

  fn version(&self) -> &Version {
    &self.version
  }

  fn can_publish(&self) -> bool {
    let publish = self
      .doc
      .get("package")
      .and_then(|x| x.get("publish").and_then(|x| x.as_bool()));
    if let Some(false) = publish {
      return false;
    }
    true
  }

  fn write(&self, next_version: &Version) -> Result<Vec<Actions>, Error> {
    let mut doc = self.doc.clone();
    write_cargo_version(&mut doc, next_version, None);
    Ok(vec![Actions::Write {
      path: self.path.to_owned(),
      content: doc.to_string(),
      prev_content: Some(self.doc.to_string()),
    }])
  }

  fn publish(&self, _next_version: &Version) -> Result<Vec<Actions>, Error> {
    Ok(vec![Actions::Command {
      cmd: "cargo".to_string(),
      args: vec!["publish".to_string(), "--allow-dirty".to_string()],
      path: self.path().parent().unwrap().to_owned(),
    }])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let path = RelativePathBuf::from("Cargo.toml");
    let content = r#"[package]
name = "test"
version = "0.1.0"
"#;
    let versioned_file = Cargo::new(path.clone(), content.to_string()).unwrap();
    assert_eq!(versioned_file.path(), &path);
    assert_eq!(versioned_file.version(), &Version::parse("0.1.0").unwrap());
    assert!(versioned_file.can_publish());
  }

  #[test]
  fn invalid() {
    let path = RelativePathBuf::from("Cargo.toml");
    let content = r#"[package]
name = "test"
"#;
    let result = Cargo::new(path.clone(), content.to_string());
    assert!(result.is_err());
  }

  #[test]
  fn cannot_publish() {
    let path = RelativePathBuf::from("Cargo.toml");
    let content = r#"[package]
name = "test"
version = "0.1.0"
publish = false
"#;
    let versioned_file = Cargo::new(path.clone(), content.to_string()).unwrap();
    assert!(!versioned_file.can_publish());
  }

  #[test]
  fn write() {
    let path = RelativePathBuf::from("Cargo.toml");
    let content = r#"[package]
name = "test"
version = "0.1.0"
"#;
    let versioned_file = Cargo::new(path.clone(), content.to_string()).unwrap();
    let next_version = Version::parse("0.2.0").unwrap();
    assert_eq!(
      versioned_file.write(&next_version).unwrap(),
      vec![Actions::Write {
        path: RelativePathBuf::from("Cargo.toml"),
        content: r#"[package]
name = "test"
version = "0.2.0"
"#
        .to_string(),
        prev_content: Some(content.to_string()),
      }]
    );
  }

  #[test]
  fn publish() {
    let path = RelativePathBuf::from("Cargo.toml");
    let content = r#"[package]
name = "test"
version = "0.1.0"
"#;
    let versioned_file = Cargo::new(path.clone(), content.to_string()).unwrap();
    let next_version = Version::parse("0.2.0").unwrap();
    assert_eq!(
      versioned_file.publish(&next_version).unwrap(),
      vec![Actions::Command {
        path: RelativePathBuf::from(""),
        cmd: "cargo".to_string(),
        args: vec!["publish".to_string(), "--allow-dirty".to_string()],
      }]
    );
  }
}
