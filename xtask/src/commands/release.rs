use crate::Error;
use crate::actions::run_actions;
use crate::cargo::write_cargo_workspace_version;
use crate::changelog::Changelog;
use crate::changes::Changes;
use crate::config::Config;
use crate::package::Package;
use biome_console::{Console, ConsoleExt, markup};
use git2::{IndexAddOption, ObjectType, Repository, Signature};
use relative_path::RelativePathBuf;
use similar::DiffableStr;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn release<C>(root_dir: &Path, console: Arc<Mutex<Box<C>>>, dry_run: bool) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  let config = Config::load(root_dir)?;
  let repo = Repository::open(root_dir)?;
  let mut targets = prepare_targets(root_dir, &repo, &config, console.clone())?;
  if targets.is_empty() {
    return Ok(());
  }

  // 1. Write files
  write_release_targets(root_dir, &mut targets, console.clone(), dry_run)?;
  write_root_cargo(root_dir, &targets, console.clone(), dry_run)?;
  write_root_changelog(root_dir, &config, &targets, console.clone(), dry_run)?;

  // 2. Publish targets
  publish_targets(root_dir, &targets, console.clone(), dry_run)?;

  // 3. Commit changes
  commit_changes(&repo, &targets, console.clone(), dry_run)?;

  // 4. Add git tag
  create_git_tags(&repo, &targets, console.clone(), dry_run)?;

  // 5. Create GitHub releases

  Ok(())
}

struct ReleaseTarget(Package, Changes, Option<Changelog>);

fn prepare_targets<C>(
  root_dir: &Path,
  repo: &Repository,
  config: &Config,
  console: Arc<Mutex<Box<C>>>,
) -> Result<Vec<ReleaseTarget>, Error>
where
  C: Console + Send + Sync + 'static,
{
  let mut targets = vec![];
  let packages = Package::load_all(root_dir, config)?;

  for pkg in packages {
    let mut pkg = pkg;
    let mut cons = console.lock().unwrap();
    let prefix = format!("[{}]", pkg.name());
    let changes = Changes::from_git_tag(repo, &pkg.versioned_git_tag(), &pkg.config().scopes)?;
    let bump_rule = changes.get_bump_rule();
    if bump_rule.is_none() {
      cons.log(markup! {
        <Info>{prefix}</Info>" No changes found. Skip release."
      });
      continue;
    }
    let bump_rule = bump_rule.unwrap();
    pkg.bump_version(&bump_rule)?;
    cons.log(markup! {
      <Info>{prefix}</Info>" "{pkg.version().to_string()}" -> "<Success>{pkg.next_version().to_string()}</Success>
    });
    let all_changes = changes.iter();
    let len = all_changes.len();
    for (i, change) in all_changes.enumerate() {
      let line = match i == len - 1 {
        true => "└─",
        false => "├─",
      };
      cons.log(markup! {
        "  "<Dim>" "{line}" "{change.to_string()}</Dim>
      });
    }
    let changelog = pkg
      .config()
      .changelog
      .as_ref()
      .map(RelativePathBuf::from)
      .map(|path| Changelog::load(root_dir, path))
      .transpose()?;
    targets.push(ReleaseTarget(pkg, changes, changelog));
  }

  Ok(targets)
}

fn write_release_targets<C>(
  root_dir: &Path,
  targets: &mut [ReleaseTarget],
  console: Arc<Mutex<Box<C>>>,
  dry_run: bool,
) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  for ReleaseTarget(pkg, changes, changelog) in targets.iter_mut() {
    let actions = pkg.write()?;
    run_actions(pkg.name(), root_dir, console.clone(), actions, dry_run)?;

    if let Some(changelog) = changelog {
      changelog.append_changes(pkg, changes);
      let actions = changelog.write();
      run_actions(pkg.name(), root_dir, console.clone(), actions, dry_run)?;
    }
  }
  Ok(())
}

fn write_root_cargo<C>(
  root_dir: &Path,
  targets: &[ReleaseTarget],
  console: Arc<Mutex<Box<C>>>,
  dry_run: bool,
) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  let packages = targets.iter().map(|x| &x.0).collect::<Vec<_>>();
  let actions = write_cargo_workspace_version(root_dir, &packages)?;
  run_actions("root", root_dir, console.clone(), actions, dry_run)?;
  Ok(())
}

fn write_root_changelog<C>(
  root_dir: &Path,
  config: &Config,
  targets: &[ReleaseTarget],
  console: Arc<Mutex<Box<C>>>,
  dry_run: bool,
) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  if let Some(ref root_changelog_path) = config.root_changelog {
    let path = RelativePathBuf::from(root_changelog_path);
    let mut changelog = Changelog::load(root_dir, path)?;
    for ReleaseTarget(pkg, changes, _) in targets.iter() {
      changelog.append_changes(pkg, changes);
    }
    let actions = changelog.write();
    run_actions("root", root_dir, console.clone(), actions, dry_run)?;
  }
  Ok(())
}

fn publish_targets<C>(
  root_dir: &Path,
  targets: &[ReleaseTarget],
  console: Arc<Mutex<Box<C>>>,
  dry_run: bool,
) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  for ReleaseTarget(pkg, _, __) in targets.iter() {
    let actions = pkg.publish()?;
    run_actions(pkg.name(), root_dir, console.clone(), actions, dry_run)?;
  }
  Ok(())
}

fn commit_changes<C>(
  repo: &Repository,
  targets: &[ReleaseTarget],
  console: Arc<Mutex<Box<C>>>,
  dry_run: bool,
) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  let message = "release commit [skip actions]";
  if dry_run {
    console.lock().unwrap().log(markup! {
      <Info>"[root]"</Info>" Will commit changes with message: "<Dim>{message}</Dim>
    });
    return Ok(());
  }
  let mut pathspecs = targets
    .iter()
    .flat_map(|x| x.0.versioned_files())
    .map(|x| x.path().to_string())
    .collect::<Vec<_>>();
  pathspecs.push("Cargo.toml".to_string());

  let mut index = repo.index()?;
  index.add_all(pathspecs, IndexAddOption::DEFAULT, None)?;

  let tree_id = index.write_tree()?;
  let tree = repo.find_tree(tree_id)?;
  let sig = Signature::now("Seokju Na", "seokju.me@gmail.com")?;
  let parent = repo.head()?.peel_to_commit()?;
  let commit_id = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent])?;
  console.lock().unwrap().log(markup! {
    <Success>"[root]"</Success>" Commit release completed: "<Dim>{commit_id.to_string()}</Dim>
  });
  Ok(())
}

fn create_git_tags<C>(
  repo: &Repository,
  targets: &[ReleaseTarget],
  console: Arc<Mutex<Box<C>>>,
  dry_run: bool,
) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  let sig = Signature::now("Seokju Na", "seokju.me@gmail.com")?;
  let target = repo.head()?.peel(ObjectType::Commit)?;
  let target_id = target.id().to_string();
  let target_id = target_id.slice(0..7);
  for ReleaseTarget(pkg, _, __) in targets.iter() {
    let mut cons = console.lock().unwrap();
    let prefix = format!("[{}]", pkg.name());
    let tag = pkg.next_versioned_git_tag();
    let tag_name = tag.tag_name();
    if dry_run {
      cons.log(markup! {
        <Info>{prefix}</Info>" Will create git tag ("{target_id}"):"<Dim>{tag_name}</Dim>
      });
      continue;
    }
    let tag_id = repo.tag(&tag_name, &target, &sig, &tag_name, false)?;
    console.lock().unwrap().log(markup! {
    <Success>{prefix}</Success>" Tag created: "<Dim>{tag_id.to_string()}</Dim>
    });
  }
  Ok(())
}
