use crate::Error;
use crate::actions::{Actions, run_actions};
use crate::cargo::write_cargo_workspace_version;
use crate::changelog::Changelog;
use crate::changes::Changes;
use crate::commands::PrereleaseOptions;
use crate::config::Config;
use crate::github::{GitHubClient, GitHubCreateReleasePayload};
use crate::package::Package;
use crate::version::BumpRule;
use biome_console::{Console, ConsoleExt, markup};
use git2::{Cred, IndexAddOption, ObjectType, PushOptions, RemoteCallbacks, Repository, Signature};
use relative_path::RelativePathBuf;
use similar::DiffableStr;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn release<C>(
  root_dir: &Path,
  console: Arc<Mutex<Box<C>>>,
  prerelease: Option<PrereleaseOptions>,
  github_token: Option<String>,
  dry_run: bool,
) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  let config = Config::load(root_dir)?;
  let repo = Repository::open(root_dir)?;
  let mut targets = prepare_targets(
    root_dir,
    &repo,
    &config,
    prerelease.as_ref(),
    console.clone(),
  )?;
  if targets.is_empty() {
    return Ok(());
  }

  // 1. Write files
  write_release_targets(root_dir, &mut targets, console.clone(), dry_run)?;
  write_root_cargo(root_dir, &targets, console.clone(), dry_run)?;
  write_root_changelog(root_dir, &config, &targets, console.clone(), dry_run)?;

  // 2. Publish targets
  publish_targets(root_dir, &targets, console.clone(), dry_run)?;

  if prerelease.is_none() {
    // 3. Commit changes
    git_commit_changes(&repo, &config, &targets, console.clone(), dry_run)?;

    // 4. Add git tag
    git_create_tags(&repo, &targets, console.clone(), dry_run)?;

    // 5. Push
    git_push(&repo, &github_token, console.clone(), dry_run)?;

    // 5. Create GitHub releases
    create_github_releases(&config, &targets, &github_token, console.clone(), dry_run)?;
  }

  Ok(())
}

struct ReleaseTarget(Package, Changes, Option<Changelog>);

fn prepare_targets<C>(
  root_dir: &Path,
  repo: &Repository,
  config: &Config,
  prerelease: Option<&PrereleaseOptions>,
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
    let bump_rule = match prerelease {
      Some(opts) => BumpRule::Prerelease {
        id: opts.id.to_owned(),
        num: opts.num,
      },
      None => bump_rule.unwrap(),
    };
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
    if let Some(ref scripts) = pkg.config().before_publish_scripts {
      let actions = scripts
        .iter()
        .map(|x| Actions::Command {
          cmd: x.command.to_owned(),
          args: x.args.to_owned().unwrap_or_default(),
          path: RelativePathBuf::from(x.cwd.to_owned().unwrap_or_default()),
        })
        .collect::<Vec<_>>();
      run_actions(pkg.name(), root_dir, console.clone(), actions, dry_run)?;
    }

    let actions = pkg.publish()?;
    run_actions(pkg.name(), root_dir, console.clone(), actions, dry_run)?;
  }
  Ok(())
}

fn git_commit_changes<C>(
  repo: &Repository,
  config: &Config,
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
      <Info>"[root]"</Info>" Will commit changes with message: "{message}
    });
    return Ok(());
  }

  let mut pathspecs = vec!["Cargo.toml".to_string()];
  for ReleaseTarget(pkg, _, changelog) in targets.iter() {
    for versioned_file in pkg.versioned_files() {
      pathspecs.push(versioned_file.path().to_string());
    }
    if let Some(changelog) = changelog {
      pathspecs.push(changelog.path().to_string());
    }
  }
  if let Some(ref root_changelog_path) = config.root_changelog {
    pathspecs.push(root_changelog_path.to_owned());
  }

  let mut cons = console.lock().unwrap();

  let mut index = repo.index()?;
  index.add_all(pathspecs, IndexAddOption::DEFAULT, None)?;

  let tree_id = index.write_tree()?;
  let tree = repo.find_tree(tree_id)?;
  let sig = Signature::now("Seokju Na", "seokju.me@gmail.com")?;
  let parent = repo.head()?.peel_to_commit()?;
  let commit_id = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent])?;
  let commit = repo.find_commit(commit_id)?;
  cons.log(markup! {
    <Success>"[root]"</Success>" Commit changes with \""{message}"\"\n"
    <Dim>"  sha: "{commit_id.to_string()}</Dim>"\n"
    <Dim>"  author name: "{commit.author().name().unwrap_or_default()}</Dim>"\n"
    <Dim>"  author email: "{commit.author().email().unwrap_or_default()}</Dim>
  });
  Ok(())
}

fn git_create_tags<C>(
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
        <Info>{prefix}</Info>" Will create git tag ("{target_id}"): "{tag_name}
      });
      continue;
    }
    let tag_id = repo.tag(&tag_name, &target, &sig, &tag_name, false)?;
    let tag = repo.find_tag(tag_id)?;
    cons.log(markup! {
      <Success>{prefix}</Success>" Tag created with name \""{tag_name}"\"\n"
      <Dim>"  sha: "{tag_id.to_string()}</Dim>"\n"
      <Dim>"  message: "{tag.message().unwrap_or_default()}</Dim>"\n"
      <Dim>"  tagger name: "{tag.tagger().and_then(|x| x.name().map(|x| x.to_string())).unwrap_or_default()}</Dim>"\n"
      <Dim>"  tagger email: "{tag.tagger().and_then(|x| x.email().map(|x| x.to_string())).unwrap_or_default()}</Dim>
    });
  }
  Ok(())
}

fn git_push<C>(
  repo: &Repository,
  github_token: &Option<String>,
  console: Arc<Mutex<Box<C>>>,
  dry_run: bool,
) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  let mut cons = console.lock().unwrap();
  let mut remote = repo.find_remote("origin")?;
  if dry_run {
    cons.log(markup! {
      <Info>"[root]"</Info>" Will push changes to remote: "{remote.url().unwrap_or_default()}
    });
    return Ok(());
  }

  let mut callbacks = RemoteCallbacks::new();
  callbacks.credentials(|_, username, _| {
    Cred::userpass_plaintext(
      username.unwrap_or_default(),
      github_token.clone().unwrap_or_default().as_ref(),
    )
  });

  let mut push_opts = PushOptions::default();
  push_opts.remote_callbacks(callbacks);
  push_opts.remote_push_options(&["--tags"]);

  remote.push(&["refs/heads/main:refs/heads/main"], Some(&mut push_opts))?;
  cons.log(markup! {
    <Success>"[root]"</Success>" Push changes to remote: "{remote.url().unwrap_or_default()}
  });

  Ok(())
}

fn create_github_releases<C>(
  config: &Config,
  targets: &[ReleaseTarget],
  github_token: &Option<String>,
  console: Arc<Mutex<Box<C>>>,
  dry_run: bool,
) -> Result<(), Error>
where
  C: Console + Send + Sync + 'static,
{
  for ReleaseTarget(pkg, _, changelog) in targets.iter() {
    let mut cons = console.lock().unwrap();
    let body = changelog.as_ref().and_then(|x| x.extract_changes(pkg));
    let payload = GitHubCreateReleasePayload {
      tag_name: pkg.next_versioned_git_tag().tag_name(),
      name: Some(format!("{} v{}", pkg.name(), pkg.next_version())),
      body,
    };
    if dry_run {
      cons.log(markup! {
        <Info>"[root]"</Info>" Will create GitHub release: "{config.github.repo.owner}"/"{config.github.repo.name}
      });
      for line in serde_json::to_string_pretty(&payload)?.lines() {
        cons.log(markup! {
          <Dim>"  "{line}</Dim>
        });
      }
    } else {
      let client = GitHubClient::new(github_token)?;
      let release = client.create_release(
        &config.github.repo.owner,
        &config.github.repo.name,
        &payload,
      )?;
      cons.log(markup! {
        <Success>"[root]"</Success>" Created GitHub release: "{config.github.repo.owner}"/"{config.github.repo.name}"\n"
        <Dim>"  id: "{release.id}</Dim>"\n"
        <Dim>"  url: "{release.html_url}</Dim>"\n"
        <Dim>"  tag_name: "{release.tag_name}</Dim>
      });
    }
  }
  Ok(())
}
