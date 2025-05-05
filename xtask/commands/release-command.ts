import fs from 'node:fs/promises';
import path from 'node:path';
import { Octokit } from '@octokit/rest';
import { Command, Option } from 'clipanion';
import { type Repository, openRepository } from 'es-git';
import { isNotNil } from 'es-toolkit';
import { type Action, runActions } from '../action.ts';
import { editCargoTomlVersion, formatCargoToml, parseCargoToml } from '../cargo-toml.ts';
import { Changelog } from '../changelog.ts';
import { Changes } from '../changes.ts';
import { type Config, loadConfig } from '../config.ts';
import { ColorModeOption, colors, setColorMode } from '../console.ts';
import { ROOT_DIR } from '../consts.ts';
import { Package } from '../package.ts';
import { parsePrerelease } from '../version.ts';

interface ReleaseTarget {
  package: Package;
  changes: Changes;
  changelog: Changelog | null;
}

export class ReleaseCommand extends Command {
  static paths = [['release']];

  readonly configFilepath = Option.String('--config', 'xtask.json');
  readonly dryRun = Option.Boolean('--dry-run', false);
  readonly githubToken = Option.String('--github-token', {
    required: false,
    env: 'GITHUB_TOKEN',
  });
  readonly prerelease = Option.String('--prerelease');
  readonly colorMode = ColorModeOption;

  async execute() {
    setColorMode(this.colorMode);
    try {
      const config = await loadConfig(this.configFilepath);
      const repo = await openRepository(ROOT_DIR);
      const targets = await this.prepareTargets(repo, config);
      if (targets.length === 0) {
        return 0;
      }

      await this.writeReleaseTarget(targets);
      const rootCargoChanged = await this.writeRootCargoToml(targets);
      const rootChangelogChanged = await this.writeRootChangelog(config, targets);

      await this.publish(targets);

      if (this.prerelease == null) {
        this.gitCommitChanges(repo, config, targets, rootCargoChanged, rootChangelogChanged);
        this.gitCreateTags(repo, targets);
        await this.gitPush(repo, targets);
        await this.createGitHubReleases(config, targets);
      }

      return 0;
    } catch (e) {
      console.error(e);
      return 1;
    }
  }

  async prepareTargets(repo: Repository, config: Config) {
    const targets: ReleaseTarget[] = [];
    const packages = await Package.loadAll(config);
    for (const pkg of packages) {
      const prefix = `[${pkg.name}]`;
      const changes = Changes.tryFromGitTag(repo, pkg.versionedGitTag, pkg.config.scopes);
      let bumpRule = changes.getBumpRule();
      if (bumpRule == null) {
        console.log(`${colors.warn(prefix)} no changes found. skip release.`);
        continue;
      }
      if (this.prerelease != null) {
        const prerelease = parsePrerelease(this.prerelease);
        bumpRule = { type: 'prerelease', data: prerelease };
      }
      pkg.bumpVersion(bumpRule);
      console.log(`${colors.info(prefix)} ${pkg.version.toString()} -> ${colors.success(pkg.nextVersion.toString())}`);
      for (let i = 0; i < changes.changes.length; i += 1) {
        const change = changes.changes[i]!;
        const line = i === changes.changes.length - 1 ? '└─' : '├─';
        console.log(`   ${colors.dim(`${line} ${change.toString()}`)}`);
      }
      const changelog = pkg.config.changelog != null ? await Changelog.load(pkg.config.changelog) : null;
      targets.push({ package: pkg, changes, changelog });
    }
    return targets;
  }

  async writeReleaseTarget(targets: ReleaseTarget[]) {
    for (const target of targets) {
      const actions = target.package.write();
      await runActions(actions, { name: target.package.name, dryRun: this.dryRun });

      if (target.changelog != null) {
        target.changelog.appendChanges(target.package, target.changes);
        const actions = target.changelog.write();
        await runActions(actions, { name: target.package.name, dryRun: this.dryRun });
      }
    }
  }

  async writeRootCargoToml(targets: ReleaseTarget[]) {
    const hasCargoChanged = targets
      .filter(x => x.package.hasChanged)
      .flatMap(x => x.package.versionedFiles)
      .some(x => x.type === 'Cargo.toml');
    if (!hasCargoChanged) {
      return false;
    }
    const raw = await fs.readFile(path.join(ROOT_DIR, 'Cargo.toml'), 'utf8');
    const toml = parseCargoToml(raw);
    for (const target of targets) {
      for (const versionedFile of target.package.versionedFiles) {
        if (versionedFile.type !== 'Cargo.toml') {
          continue;
        }
        editCargoTomlVersion(toml, versionedFile.nextVersion.toString(), versionedFile.name);
      }
    }
    await runActions(
      [
        {
          type: 'write',
          path: 'Cargo.toml',
          content: formatCargoToml(toml),
          prevContent: raw,
        },
      ],
      { dryRun: this.dryRun }
    );
    return true;
  }

  async writeRootChangelog(config: Config, targets: ReleaseTarget[]) {
    if (config.rootChangelog == null) {
      return false;
    }
    const changelog = await Changelog.load(config.rootChangelog);
    for (const target of targets) {
      changelog.appendChanges(target.package, target.changes);
    }
    await runActions(changelog.write(), { dryRun: this.dryRun });
    return true;
  }

  async publish(targets: ReleaseTarget[]) {
    for (const target of targets) {
      if (target.package.config.beforePublishScripts != null) {
        const actions = target.package.config.beforePublishScripts.map(
          (script): Action => ({
            type: 'command',
            cmd: script.command,
            args: (script.args ?? []) as string[],
            path: script.cwd ?? '',
          })
        );
        await runActions(actions, { name: target.package.name, dryRun: this.dryRun });
      }
      const actions = target.package.publish();
      await runActions(actions, { name: target.package.name, dryRun: this.dryRun });
    }
  }

  gitCommitChanges(
    repo: Repository,
    config: Config,
    targets: ReleaseTarget[],
    rootCargoChanged: boolean,
    rootChangelogChanged: boolean
  ) {
    const message = 'release commit [skip actions]';
    if (this.dryRun) {
      console.log(`${colors.info('[root]')} will commit changes: ${message}`);
      return;
    }
    const pathspecs = targets.flatMap(x =>
      [...x.package.versionedFiles.map(x => x.path), x.changelog?.path].filter(isNotNil)
    );
    if (rootCargoChanged) {
      pathspecs.push('Cargo.toml');
    }
    if (rootChangelogChanged && config.rootChangelog != null) {
      pathspecs.push(config.rootChangelog);
    }
    const index = repo.index();
    index.addAll(pathspecs);

    const treeId = index.writeTree();
    const tree = repo.getTree(treeId);
    const parent = repo.head().target()!;
    const sig = { name: 'Seokju Na', email: 'seokju.me@gmail.com' };
    const commitId = repo.commit(tree, message, {
      updateRef: 'HEAD',
      author: sig,
      committer: sig,
      parents: [parent],
    });
    const commit = repo.getCommit(commitId);
    console.log(`${colors.info('[root]')} commit: ${message}`);
    console.log(colors.dim(`  sha: ${commit.id()}`));
    console.log(colors.dim(`  author name: ${commit.author().name}`));
    console.log(colors.dim(`  author email: ${commit.author().email}`));
  }

  private gitCreateTags(repo: Repository, targets: ReleaseTarget[]) {
    const sig = { name: 'Seokju Na', email: 'seokju.me@gmail.com' };
    const target = repo.head().target()!;
    const commit = repo.getCommit(target);
    const targetId = commit.id().slice(0, 7);
    for (const target of targets) {
      const prefix = `[${target.package.name}]`;
      const tag = target.package.nextVersionedGitTag;
      if (this.dryRun) {
        console.log(`${colors.info(prefix)} will create tag (${targetId}): ${tag.tagName}`);
        continue;
      }
      const tagId = repo.createTag(tag.tagName, commit.asObject(), tag.tagName, { tagger: sig });
      const gitTag = repo.getTag(tagId);
      console.log(`${colors.info('root')} tag: ${gitTag.name()}`);
      console.log(colors.dim(`  sha: ${gitTag.id()}`));
      console.log(colors.dim(`  message: ${gitTag.message()}`));
      console.log(colors.dim(`  tagger name: ${gitTag.tagger()?.name}`));
      console.log(colors.dim(`  tagger email: ${gitTag.tagger()?.email}`));
    }
  }

  private async gitPush(repo: Repository, targets: ReleaseTarget[]) {
    const remote = repo.getRemote('origin');
    const refspecs = [
      'refs/heads/main:refs/heads/main',
      ...targets.map(x => x.package.nextVersionedGitTag.tagRef).map(tagRef => `${tagRef}:${tagRef}`),
    ];
    const logRefspecs = () => {
      for (const refspec of refspecs) {
        const [src, dest] = refspec.split(':');
        console.log(colors.dim(`  - ${src} -> ${dest}`));
      }
    };
    if (this.dryRun) {
      console.log(`${colors.info('[root]')} will push to: ${remote.url()}`);
      logRefspecs();
      return;
    }
    if (this.githubToken == null) {
      console.log(`${colors.warn('[root]')} no github token found. skip push.`);
      return;
    }
    await remote.push(
      [
        'refs/heads/main:refs/heads/main',
        ...targets.map(x => x.package.nextVersionedGitTag.tagRef).map(tagRef => `${tagRef}:${tagRef}`),
      ],
      {
        credential: {
          type: 'Plain',
          password: this.githubToken,
        },
      }
    );
    console.log(`${colors.success('[root]')} push changes to remote: ${remote.url()}`);
    logRefspecs();
  }

  private async createGitHubReleases(config: Config, targets: ReleaseTarget[]) {
    const { github } = config;
    const client = new Octokit({
      auth: this.githubToken,
      userAgent: 'webview-bundle',
    });
    for (const target of targets) {
      const payload = {
        tag_name: target.package.nextVersionedGitTag.tagName,
        name: `${target.package.name} v${target.package.nextVersion.toString()}`,
        body: target.changelog?.extractChanges(target.package) ?? undefined,
      };
      if (this.dryRun) {
        console.log(`${colors.info('[root]')} will create github release: ${github.repo.owner}/${github.repo.name}`);
        const payloadStr = JSON.stringify(payload, null, 2);
        for (const line of payloadStr.split('\n')) {
          console.log(`  ${colors.dim(line)}`);
        }
        continue;
      }
      if (this.githubToken == null) {
        console.log(`${colors.warn('[root]')} no github token found. skip creating github release.`);
        return;
      }
      const release = await client.rest.repos.createRelease({
        owner: github.repo.owner,
        repo: github.repo.name,
        ...payload,
      });
      console.log(`${colors.success('[root]')} create github release: ${release.data.tag_name}`);
      console.log(`  ${colors.dim(`id: ${release.data.id}`)}`);
      console.log(`  ${colors.dim(`html_url: ${release.data.html_url}`)}`);
    }
  }
}
