use crate::version::Version;
use git2::Oid;

pub struct VersionedGitTag {
  name: String,
  version: Version,
}

impl VersionedGitTag {
  pub fn new(name: &str, version: Version) -> Self {
    Self {
      name: name.to_string(),
      version,
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn version(&self) -> &Version {
    &self.version
  }

  pub fn tag_name(&self) -> String {
    format!("{}/v{}", self.name, self.version)
  }

  pub fn tag_ref(&self) -> String {
    format!("refs/tags/{}", self.tag_name())
  }

  pub fn find_tag<'a>(
    &self,
    repo: &'a git2::Repository,
  ) -> Result<Option<git2::Tag<'a>>, crate::Error> {
    let mut oid: Option<Oid> = None;
    repo.tag_foreach(|tag_oid, name| {
      if let Ok(tag_name) = std::str::from_utf8(name) {
        if tag_name == self.tag_name() {
          oid = Some(tag_oid);
          return false;
        }
      }
      true
    })?;
    let tag = oid.map(|x| repo.find_tag(x)).transpose()?;
    Ok(tag)
  }
}
