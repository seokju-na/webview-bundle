use crate::Error;
use crate::actions::Actions;
use crate::package::Package;
use crate::version::Version;
use crate::versioned_file::VersionedFileKind;
use relative_path::RelativePathBuf;
use std::path::Path;
use std::str::FromStr;
use toml_edit::{DocumentMut, Item, value};

pub fn write_cargo_version(doc: &mut DocumentMut, version: &Version, dep: Option<&str>) {
  if let Some(dep) = dep {
    if let Some(item) = doc
      .get_mut("dependencies")
      .and_then(|deps| deps.get_mut(dep))
    {
      write_cargo_version_item(item, version);
    }
    if let Some(item) = doc
      .get_mut("dev-dependencies")
      .and_then(|deps| deps.get_mut(dep))
    {
      write_cargo_version_item(item, version);
    }
    if let Some(item) = doc
      .get_mut("workspace")
      .and_then(|workspace| workspace.get_mut("dependencies")?.get_mut(dep))
    {
      write_cargo_version_item(item, version);
    }
  } else {
    let item = doc
      .get_mut("package")
      .and_then(|package| package.get_mut("version"));
    if let Some(val) = item {
      *val = value(version.to_string());
    }
  }
}

fn write_cargo_version_item(item: &mut Item, version: &Version) {
  let ver = version.to_string();
  if let Some(table) = item.as_table_mut() {
    table.insert("version", value(ver));
  } else if let Some(table) = item.as_inline_table_mut() {
    table.insert("version", ver.into());
  } else if let Some(value) = item.as_value_mut() {
    *value = ver.into();
  }
}

pub fn write_cargo_workspace_version(
  root_dir: &Path,
  packages: &[&Package],
) -> Result<Vec<Actions>, Error> {
  let has_cargo_changed = packages
    .iter()
    .flat_map(|x| x.versioned_files())
    .any(|x| x.has_changed() && x.kind() == &VersionedFileKind::Cargo);

  if !has_cargo_changed {
    return Ok(vec![]);
  }

  let filepath = root_dir.join("Cargo.toml");
  let content = std::fs::read_to_string(&filepath)?;
  let mut doc = DocumentMut::from_str(&content)?;

  for pkg in packages {
    if !pkg.has_changed() {
      continue;
    }
    for versioned_file in pkg.versioned_files() {
      if versioned_file.kind() != &VersionedFileKind::Cargo {
        continue;
      }
      write_cargo_version(
        &mut doc,
        versioned_file.next_version(),
        Some(versioned_file.name()),
      );
    }
  }

  Ok(vec![Actions::Write {
    path: RelativePathBuf::from("Cargo.toml"),
    content: doc.to_string(),
    prev_content: Some(content),
  }])
}

pub fn cargo_pkg_name(doc: &DocumentMut) -> Option<&str> {
  doc
    .get("package")
    .and_then(|package| package.get("name")?.as_str())
}

pub fn cargo_pkg_version(doc: &DocumentMut) -> Option<&str> {
  doc
    .get("package")
    .and_then(|package| package.get("version")?.as_str())
}
