import path from 'node:path';
import chalk from 'chalk';
import { Command, Option } from 'clipanion';
import * as gitUtils from '../gitUtils';
import * as workspaceUtils from '../workspaceUtils';
import { type Action, formatAction, runAction } from './action';
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

      allActions.push(...pkg.writeVersionedFiles());

      // Changelog
      const changelog = await Changelog.load(info.changelog);
      changelog.appendChanges(`v${pkg.nextVersion.format()}`, changes);

      allActions.push(changelog.write());
      if (rootChangelog != null) {
        rootChangelog.appendChanges(`${pkg.name} v${pkg.nextVersion.format()}`, changes);
      }
    }

    allActions.push(rootChangelog?.write());

    const actions = allActions.filter(x => x != null);
    if (actions.length === 0) {
      this.context.stderr.write(chalk.red('Nothing to release'));
      this.context.stderr.write('\n');
      return 1;
    }

    for (const action of actions) {
      this.context.stdout.write(formatAction(action));
      this.context.stdout.write('\n\n');
      if (!this.dryRun) {
        await runAction(action);
      }
    }
  }
}
