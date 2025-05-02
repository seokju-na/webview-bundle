use crate::Error;
use crate::actions::Actions;
use crate::changes::Changes;
use crate::package::Package;
use crate::versioned_file::{VersionedFile, VersionedFileKind};
use relative_path::RelativePathBuf;
use std::path::Path;

pub struct Changelog {
  path: RelativePathBuf,
  raw: String,
  lines: Vec<String>,
}

impl Changelog {
  pub fn new(path: RelativePathBuf, content: String) -> Self {
    let lines = content.lines().map(|x| x.to_string()).collect::<Vec<_>>();
    Self {
      path,
      raw: content,
      lines,
    }
  }

  pub fn load(root_dir: &Path, path: RelativePathBuf) -> Result<Self, Error> {
    let filepath = path.to_path(root_dir);
    let content = std::fs::read_to_string(filepath)?;
    Ok(Changelog::new(path, content))
  }

  pub fn has_changed(&self) -> bool {
    self.raw != self.new_content()
  }

  pub fn append_changes(&mut self, package: &Package, changes: &Changes) {
    if !package.has_changed() || package.next_version().is_prerelease() {
      return;
    }
    let title = format!("## {} v{}", package.name(), package.next_version());
    let mut changes_lines = changes_lines(changes);
    let idx = self.lines.iter().position(|x| x == &title);
    match idx {
      Some(idx) => {
        let i = idx + 2;
        self.lines.splice(i..i, changes_lines);
      }
      None => {
        let pkg_names = package
          .versioned_files()
          .iter()
          .map(format_versioned_file_pkg_name)
          .collect::<Vec<_>>()
          .join(", ");
        changes_lines.insert(
          0,
          format!(
            r#"
{title}

This release includes packages: {pkg_names}
"#
          ),
        );
        self.lines.splice(1..1, changes_lines);
      }
    }
  }

  pub fn write(&self) -> Vec<Actions> {
    if !self.has_changed() {
      return vec![];
    }
    vec![Actions::Write {
      path: self.path.to_owned(),
      content: self.new_content(),
      prev_content: Some(self.raw.to_owned()),
    }]
  }

  fn new_content(&self) -> String {
    format!("{}\n", self.lines.join("\n"))
  }
}

fn changes_lines(changes: &Changes) -> Vec<String> {
  let mut lines = vec![];
  for change in changes.iter() {
    let line = format!("- {}", change);
    lines.push(line);
  }
  lines
}

fn format_versioned_file_pkg_name(versioned_file: &VersionedFile) -> String {
  let name = versioned_file.name();
  let version = versioned_file.next_version().to_string();
  let link = match versioned_file.kind() {
    VersionedFileKind::Cargo => format!("https://crates.io/crates/{}/{}", name, version),
    VersionedFileKind::PackageJson => {
      format!("https://www.npmjs.com/package/{}/v/{}", name, version)
    }
  };
  format!("[`{}`]({})", name, link)
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::changes::Change;
//   use crate::conventional_commit::ConventionalCommit;
//
//   #[test]
//   fn test1() {
//     let changes = Changes::new(vec![
//       Change::new(
//         ConventionalCommit::parse("sha1", "feat(test): Create something").unwrap(),
//         None,
//         None,
//       ),
//       Change::new(
//         ConventionalCommit::parse("sha2", "fix(test): Fix something").unwrap(),
//         None,
//         None,
//       ),
//     ]);
//     let mut changelog = Changelog::new(
//       RelativePathBuf::from("CHANGELOG.md"),
//       "# Changelog\n".to_string(),
//     );
//     assert!(!changelog.has_changed());
//     changelog.append_changes("v1.0.0", &changes);
//     assert!(changelog.has_changed());
//     assert_eq!(
//       changelog.write(),
//       vec![Actions::Write {
//         path: RelativePathBuf::from("CHANGELOG.md"),
//         content: r#"# Changelog
//
// ## v1.0.0
//
// - feat(test): Create something
// - fix(test): Fix something
// "#
//         .to_string(),
//         prev_content: Some("# Changelog\n".to_string())
//       }]
//     )
//   }
// }
