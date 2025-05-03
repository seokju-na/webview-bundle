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

  pub fn path(&self) -> &RelativePathBuf {
    &self.path
  }

  pub fn has_changed(&self) -> bool {
    self.raw != self.new_content()
  }

  pub fn append_changes(&mut self, package: &Package, changes: &Changes) {
    if !package.has_changed() || package.next_version().is_prerelease() {
      return;
    }
    let title = Self::format_title(package);
    let changes_lines = changes_lines(changes);
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
        let content = vec![
          "".to_string(),
          title,
          "".to_string(),
          format!("This release includes packages: {}", pkg_names),
          "".to_string(),
        ];
        self.lines.splice(1..1, [content, changes_lines].concat());
      }
    }
  }

  pub fn extract_changes(&self, package: &Package) -> Option<String> {
    let mut lines = vec![];
    let title = Self::format_title(package);
    let pos = self.lines.iter().position(|x| x.starts_with(&title));

    if let Some(mut pos) = pos {
      pos += 1;
      while let Some(line) = self.lines.get(pos) {
        if line.starts_with("## ") {
          break;
        }
        lines.push(line.to_string());
        pos += 1;
      }
    }

    match lines.is_empty() {
      true => None,
      false => Some(lines.join("\n")),
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

  fn format_title(package: &Package) -> String {
    format!("## {} v{}", package.name(), package.next_version())
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
