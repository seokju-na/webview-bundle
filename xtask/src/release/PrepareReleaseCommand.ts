import fs from 'node:fs/promises';
import path from 'node:path';
import type { Repository } from '@napi-rs/simple-git';
import { Command, Option } from 'clipanion';
import { execa, execaCommand } from 'execa';
import * as gitUtils from '../gitUtils';
import * as workspaceUtils from '../workspaceUtils';
import { type Action, formatAction } from './action';
import { Changelog } from './changelog';
import { Changes } from './changes';
import { ReleaseConfig } from './config';
import { Package } from './package';

export class PrepareReleaseCommand extends Command {
  static paths = [['prepare-release']];

  dryRun = Option.Boolean('--dry-run', false, {
    description: "Perform dry run. Don't change any files or any network calls.",
  });

  async execute() {
    const rootDir = workspaceUtils.findRootDir();
    const config = await ReleaseConfig.load(path.join(rootDir, 'releases.json'));

    const repo = gitUtils.openRepo(rootDir);
    const allActions: Array<Action | undefined> = [];

    const rootChangelog = config.rootChangelog != null ? await Changelog.load(config.rootChangelog) : null;

    for (const [name, info] of config.packages.entries()) {
      const pkg = await Package.load(name, info.versionedFiles, info.scopes);
      const changes = Changes.fromGitTag(repo, pkg.versionGitTag, pkg.scopes);
      if (changes == null) {
        continue;
      }

      // Versioning
      const rule = changes.calculateBumpRule(pkg.version);
      pkg.bumpVersion(rule);

      const diff = `${pkg.version.format()} -> ${pkg.nextVersion.format()}`;
      this.context.stdout.write(`Changes for "${pkg.name}": ${diff}\n\n`);

      allActions.push(...pkg.write());

      // Changelog
      const changelog = await Changelog.load(info.changelog);
      changelog.appendChanges(`v${pkg.nextVersion.format()}`, changes);

      allActions.push(changelog.write());
      if (rootChangelog != null) {
        rootChangelog.appendChanges(`${pkg.name} v${pkg.nextVersion.format()}`, changes);
      }

      // Run before release scripts
      for (const script of info.beforeReleaseScripts ?? []) {
        allActions.push({
          type: 'runCommand',
          command: script.command,
          cwd: script.cwd != null ? workspaceUtils.absolutePathFromRootDir(script.cwd) : workspaceUtils.findRootDir(),
          env: script.env,
        });
      }
    }

    allActions.push(rootChangelog?.write());

    const actions = allActions.filter(x => x != null);
    for (const action of actions) {
      this.context.stdout.write(`${formatAction(action)}\n\n`);
      if (!this.dryRun) {
        await this.runAction(action, { repo });
      }
    }

    // Add files to git index
    // TODO: `@napi-rs/simple-git` does not implement `index()` so we have to use git with CLI
    for (const action of actions.filter(x => x.type === 'writeFile')) {
      const args = ['add', action.filepath];
      if (this.dryRun) {
        args.push('--dry-run');
      }
      await execa('git', args, { cwd: rootDir, stdio: 'inherit' });
    }
    if (!this.dryRun) {
      gitUtils.commitToHead(repo, gitUtils.defaultSignature(), 'chore: release commit');
    }

    // Push to remote
  }

  private async runAction(
    action: Action,
    params: {
      repo: Repository;
    }
  ) {
    const { repo } = params;
    switch (action.type) {
      case 'writeFile': {
        await fs.writeFile(workspaceUtils.absolutePathFromRootDir(action.filepath), action.content, 'utf8');
        break;
      }
      case 'addGitTag': {
        const head = repo.head();
        const headCommit = repo.findCommit(head.target()!);
        gitUtils.addTag(
          repo,
          action.tagName,
          headCommit!.asObject(),
          gitUtils.defaultSignature(),
          action.tagName,
          true
        );
        break;
      }
      case 'runCommand': {
        await execaCommand(action.command, { cwd: action.cwd, env: action.env, stdio: 'inherit' });
        break;
      }
    }
  }
}
