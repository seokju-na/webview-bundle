use semver::Prerelease;
use serde::{Deserialize, Serialize};

pub enum BumpRule {
  Major,
  Minor,
  Patch,
  Prerelease { id: String, num: u8 },
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Version {
  ver: semver::Version,
}

impl std::fmt::Display for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.ver)
  }
}

impl<'de> Deserialize<'de> for Version {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    let version = String::deserialize(deserializer)?;
    Version::parse(&version).map_err(serde::de::Error::custom)
  }
}

impl Serialize for Version {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    let version = self.to_string();
    serializer.serialize_str(&version)
  }
}

impl Version {
  pub fn parse(raw: &str) -> Result<Self, crate::Error> {
    let ver = semver::Version::parse(raw)?;
    Ok(Self { ver })
  }

  pub fn bump(&mut self, rule: &BumpRule) -> Result<(), crate::Error> {
    match rule {
      BumpRule::Major => {
        self.ver.major += 1;
        self.ver.minor = 0;
        self.ver.patch = 0;
      }
      BumpRule::Minor => {
        self.ver.minor += 1;
        self.ver.patch = 0;
      }
      BumpRule::Patch => {
        self.ver.patch += 1;
      }
      BumpRule::Prerelease { id, num } => {
        self.ver.pre = Prerelease::new(&format!("{}.{}", id, num))?;
      }
    }
    Ok(())
  }

  pub fn is_prerelease(&self) -> bool {
    self.prerelease_id().is_some()
  }

  pub fn prerelease_id(&self) -> Option<String> {
    if self.ver.pre.is_empty() {
      return None;
    }
    let pre_str = self.ver.pre.to_string();
    let splits = pre_str.split(".").collect::<Vec<_>>();
    splits.first().map(|x| x.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    assert_eq!(Version::parse("1.2.3").unwrap().to_string(), "1.2.3");
    assert_eq!(
      Version::parse("1.2.3-beta.1").unwrap().to_string(),
      "1.2.3-beta.1"
    );
    assert!(Version::parse("not_version").is_err());
  }

  #[test]
  fn bump() {
    let mut version = Version::parse("1.2.3").unwrap();

    version.bump(&BumpRule::Patch).unwrap();
    assert_eq!(version.to_string(), "1.2.4");

    version.bump(&BumpRule::Major).unwrap();
    assert_eq!(version.to_string(), "2.0.0");

    version.bump(&BumpRule::Minor).unwrap();
    assert_eq!(version.to_string(), "2.1.0");

    version
      .bump(&BumpRule::Prerelease {
        id: "next".to_string(),
        num: 123,
      })
      .unwrap();
    assert_eq!(version.to_string(), "2.1.0-next.123");
  }

  #[test]
  fn ord() {
    assert!(Version::parse("1.2.3").unwrap() < Version::parse("1.2.4").unwrap());
  }
}
