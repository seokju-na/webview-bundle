use crate::conventional_commit::{ConventionalCommit, ConventionalCommitKind};
use crate::version::BumpRule;
use crate::versioned_git_tag::VersionedGitTag;
use git2::{Commit, Repository};
use std::fmt::Formatter;
use std::slice::Iter;
use time::OffsetDateTime;

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeAuthor {
  pub name: String,
  pub email: String,
}

#[derive(Debug)]
pub struct Change {
  pub commit: ConventionalCommit,
  pub author: Option<ChangeAuthor>,
  pub timestamp: i64,
}

impl<'a> TryFrom<&Commit<'a>> for Change {
  type Error = crate::Error;

  fn try_from(value: &Commit<'a>) -> Result<Self, Self::Error> {
    let conventional_commit = ConventionalCommit::try_from(value)?;
    let name = value.author().name().map(|x| x.to_string());
    let email = value.author().email().map(|x| x.to_string());
    let author = if let (Some(name), Some(email)) = (name, email) {
      Some(ChangeAuthor { name, email })
    } else {
      None
    };
    let timestamp = value.time().seconds();
    Ok(Self {
      commit: conventional_commit,
      author,
      timestamp,
    })
  }
}

impl Change {
  #[must_use]
  pub fn new(
    commit: ConventionalCommit,
    author: Option<ChangeAuthor>,
    timestamp: Option<i64>,
  ) -> Self {
    Self {
      commit,
      author,
      timestamp: timestamp.unwrap_or(OffsetDateTime::now_utc().unix_timestamp()),
    }
  }
}

impl std::fmt::Display for Change {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let prefix = match self.commit.is_breaking {
      true => "[BREAKING CHANGE] ",
      false => "",
    };
    let scopes = match self.commit.scopes.is_empty() {
      true => "".to_string(),
      false => format!("({})", self.commit.scopes.join(",").to_string()),
    };
    let head = format!(
      "{}{}{}: {}",
      prefix, self.commit.kind, scopes, self.commit.summary
    );
    write!(f, "{}", &head)
  }
}

#[derive(Debug)]
pub struct Changes {
  changes: Vec<Change>,
}

impl Changes {
  pub fn new(changes: Vec<Change>) -> Self {
    Self { changes }
  }

  pub fn from_git_tag(
    repo: &Repository,
    previous_tag: &VersionedGitTag,
    scopes: &[String],
  ) -> Result<Self, crate::Error> {
    let head = repo
      .head()?
      .target()
      .ok_or(crate::Error::CannotFindHeadTarget)?;
    let tag = previous_tag.find_tag(repo)?;
    let mut walk = repo.revwalk()?;
    walk.push(head)?;
    if let Some(tag) = tag {
      walk.hide(tag.id())?;
    }
    let changes = walk
      .map(|oid| oid.and_then(|x| repo.find_commit(x)))
      .filter_map(|commit| commit.ok())
      .filter_map(|commit| Change::try_from(&commit).ok())
      .filter(|change| {
        scopes
          .iter()
          .any(|scope| change.commit.scopes.contains(scope))
      })
      .collect::<Vec<_>>();
    Ok(Self::new(changes))
  }

  pub fn is_empty(&self) -> bool {
    self.changes.is_empty()
  }

  pub fn iter(&self) -> Iter<'_, Change> {
    self.changes.iter()
  }

  pub fn get_bump_rule(&self) -> Option<BumpRule> {
    if self.is_empty() {
      return None;
    }
    if self.changes.iter().any(|x| x.commit.is_breaking) {
      Some(BumpRule::Major)
    } else if self
      .changes
      .iter()
      .any(|x| x.commit.kind == ConventionalCommitKind::Feature)
    {
      Some(BumpRule::Minor)
    } else {
      Some(BumpRule::Patch)
    }
  }
}
