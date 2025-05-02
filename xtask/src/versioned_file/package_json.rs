use crate::Error;
use crate::actions::Actions;
use crate::version::Version;
use crate::versioned_file::package_manager::PackageManager;
use relative_path::RelativePathBuf;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Json {
  name: String,
  version: Version,
  private: Option<bool>,
}

pub struct PackageJson {
  path: RelativePathBuf,
  raw: String,
  json: Json,
}

impl PackageJson {
  pub fn new(path: RelativePathBuf, content: String) -> Result<Self, Error> {
    let json = serde_json::from_str::<Json>(&content)?;
    Ok(Self {
      path,
      raw: content,
      json,
    })
  }
}

impl PackageManager for PackageJson {
  fn name(&self) -> &str {
    &self.json.name
  }

  fn path(&self) -> &RelativePathBuf {
    &self.path
  }

  fn version(&self) -> &Version {
    &self.json.version
  }

  fn can_publish(&self) -> bool {
    if let Some(true) = self.json.private {
      return false;
    }
    true
  }

  fn write(&self, next_version: &Version) -> Result<Vec<Actions>, Error> {
    let re =
      regex::Regex::new(r#"(?m)^(?P<prefix>\s*"version"\s*:\s*")(?P<old>[^"]*)(?P<suffix>"\s*)"#)
        .unwrap();
    let content = re
      .replace_all(&self.raw, |caps: &regex::Captures| {
        format!("{}{}{}", &caps["prefix"], next_version, &caps["suffix"])
      })
      .to_string();

    Ok(vec![Actions::Write {
      path: self.path().to_owned(),
      content,
      prev_content: Some(self.raw.to_owned()),
    }])
  }

  fn publish(&self, next_version: &Version) -> Result<Vec<Actions>, Error> {
    let mut args = vec!["publish".to_string(), "--access=public".to_string()];
    if let Some(id) = next_version.prerelease_id() {
      args.push(format!("--tag={}", id));
    }
    Ok(vec![Actions::Command {
      cmd: "npm".to_string(),
      args,
      path: self.path().parent().unwrap().to_owned(),
    }])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let path = RelativePathBuf::from("package.json");
    let content = r#"
      {
        "version": "1.0.0"
      }
    "#;
    let versioned_file = PackageJson::new(path.clone(), content.to_string()).unwrap();
    assert_eq!(versioned_file.path(), &path);
    assert_eq!(versioned_file.version(), &Version::parse("1.0.0").unwrap());
    assert!(versioned_file.can_publish());
  }

  #[test]
  fn cannot_publish() {
    let path = RelativePathBuf::from("package.json");
    let content = r#"
      {
        "version": "1.0.0",
        "private": true
      }
    "#;
    let versioned_file = PackageJson::new(path.clone(), content.to_string()).unwrap();
    assert!(!versioned_file.can_publish());
  }

  #[test]
  fn write() {
    let path = RelativePathBuf::from("package.json");
    let content = r#"{
  "version": "1.0.0"
}
"#;
    let versioned_file = PackageJson::new(path.clone(), content.to_string()).unwrap();
    let next_version = Version::parse("1.1.0").unwrap();
    assert_eq!(
      versioned_file.write(&next_version).unwrap(),
      vec![Actions::Write {
        path: RelativePathBuf::from("package.json"),
        content: r#"{
  "version": "1.1.0"
}
"#
        .to_string(),
        prev_content: Some(content.to_string()),
      }]
    )
  }

  #[test]
  fn publish() {
    let path = RelativePathBuf::from("package.json");
    let content = r#"
      {
        "version": "1.0.0"
      }
    "#;
    let versioned_file = PackageJson::new(path.clone(), content.to_string()).unwrap();
    let next_version = Version::parse("1.1.0").unwrap();
    assert_eq!(
      versioned_file.publish(&next_version).unwrap(),
      vec![Actions::Command {
        cmd: "npm".to_string(),
        args: vec!["publish".to_string(), "--access=public".to_string()],
        path: RelativePathBuf::from("package.json"),
      }]
    )
  }

  #[test]
  fn publish_prerelease() {
    let path = RelativePathBuf::from("package.json");
    let content = r#"
      {
        "version": "1.0.0"
      }
    "#;
    let versioned_file = PackageJson::new(path.clone(), content.to_string()).unwrap();
    let next_version = Version::parse("1.1.0-next.123").unwrap();
    assert_eq!(
      versioned_file.publish(&next_version).unwrap(),
      vec![Actions::Command {
        cmd: "npm".to_string(),
        args: vec![
          "publish".to_string(),
          "--access=public".to_string(),
          "--tag=next".to_string()
        ],
        path: RelativePathBuf::from("package.json"),
      }]
    )
  }
}
