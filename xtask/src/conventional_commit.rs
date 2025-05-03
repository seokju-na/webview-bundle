use enum_iterator::{Sequence, all};
use git2::Commit;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Sequence)]
pub enum ConventionalCommitKind {
  Feature,
  Fix,
  Refactor,
  Performance,
  Test,
  Style,
  Docs,
  Build,
  Operations,
  Chore,
}

impl std::fmt::Display for ConventionalCommitKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      Self::Feature => "feat",
      Self::Fix => "fix",
      Self::Refactor => "refactor",
      Self::Performance => "perf",
      Self::Test => "test",
      Self::Style => "style",
      Self::Docs => "docs",
      Self::Build => "build",
      Self::Operations => "ops",
      Self::Chore => "chore",
    };
    write!(f, "{}", str)
  }
}

#[derive(Debug)]
pub struct ConventionalCommit {
  #[allow(dead_code)]
  pub sha: String,
  pub kind: ConventionalCommitKind,
  pub scopes: Vec<String>,
  pub is_breaking: bool,
  pub summary: String,
  #[allow(dead_code)]
  pub body: Option<String>,
}

impl ConventionalCommit {
  pub fn parse(sha: &str, message: &str) -> Result<Self, crate::Error> {
    let lines = message.lines().collect::<Vec<_>>();
    let body = lines.get(2..).map(|x| x.join("\n"));
    let re =
      regex::Regex::new(r"^(?P<kind>\w+)(?:\((?P<scopes>.+)\))?(?P<breaking>!)?: (?P<summary>.+)$")
        .unwrap();
    let caps = re
      .captures(
        lines
          .first()
          .ok_or(crate::Error::ConventionalCommitParseError)?,
      )
      .ok_or(crate::Error::ConventionalCommitParseError)?;
    let kind_val = caps
      .name("kind")
      .ok_or(crate::Error::ConventionalCommitParseError)?;
    let kind = all::<ConventionalCommitKind>()
      .find(|x| x.to_string() == kind_val.as_str())
      .ok_or(crate::Error::ConventionalCommitParseError)?;
    let summary = caps
      .name("summary")
      .ok_or(crate::Error::ConventionalCommitParseError)?
      .as_str()
      .to_string();
    let scopes = caps
      .name("scopes")
      .map(|x| {
        x.as_str()
          .split(',')
          .map(|x| x.trim().to_string())
          .collect::<Vec<_>>()
      })
      .unwrap_or_default();
    let is_breaking = caps.name("breaking").is_some() || message.contains("BREAKING CHANGE:");
    Ok(Self {
      sha: sha.to_string(),
      kind,
      scopes,
      is_breaking,
      summary,
      body,
    })
  }
}

impl<'a> TryFrom<&Commit<'a>> for ConventionalCommit {
  type Error = crate::Error;

  fn try_from(value: &Commit<'a>) -> Result<Self, Self::Error> {
    let sha = value.id().to_string();
    let message = value
      .message()
      .ok_or(crate::Error::ConventionalCommitParseError)?;
    Self::parse(&sha, message)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    let commit = ConventionalCommit::parse(
      "123",
      r#"feat(core): implement `my-module` crate

This commit implements `my-module` crate.
"#,
    );
    assert!(commit.is_ok());
    let commit = commit.unwrap();
    assert_eq!(commit.sha, "123");
    assert_eq!(commit.kind, ConventionalCommitKind::Feature);
    assert_eq!(commit.scopes, vec!["core"]);
    assert!(!commit.is_breaking);
    assert_eq!(commit.summary, "implement `my-module` crate");
    assert_eq!(
      commit.body.unwrap(),
      "This commit implements `my-module` crate."
    );
  }

  #[test]
  fn multiple_scopes() {
    let commit = ConventionalCommit::parse("123", r#"feat(core, cli): hotfix"#).unwrap();
    assert_eq!(commit.scopes, vec!["core", "cli"]);
  }

  #[test]
  fn breaking() {
    let commit =
      ConventionalCommit::parse("123", r#"feat(core)!: implement `my-module` crate"#).unwrap();
    assert!(commit.is_breaking);
    let commit = ConventionalCommit::parse(
      "123",
      r#"feat(core): implement `my-module` crate

BREAKING CHANGE: remove '.addSomething()' method.
"#,
    )
    .unwrap();
    assert!(commit.is_breaking);
  }

  #[test]
  fn parse_fail() {
    assert!(ConventionalCommit::parse("123", "any commit message").is_err());
    assert!(ConventionalCommit::parse("123", "unknown: any commit message").is_err());
  }
}
