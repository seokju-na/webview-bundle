import path from 'node:path';
import { Command, Option } from 'clipanion';
import * as gitUtils from '../gitUtils';
import * as workspaceUtils from '../workspaceUtils';
import { calcBumpRuleFromChanges, getChangesFromGit } from './change';
import { ReleaseConfig } from './config';
import { Package } from './package';

export class ReleaseCommand extends Command {
  static paths = [['release']];

  dryRun = Option.Boolean('--dry-run', false, {
    description: "Perform dry run. Don't change any files or any network calls.",
  });

  async execute() {
    const rootDir = workspaceUtils.findRootDir();
    const config = await ReleaseConfig.load(path.join(rootDir, 'releases.json'));

    const repo = gitUtils.openRepo(rootDir);
    const packages: Package[] = [];

    for (const [name, pkgInfo] of config.packages.entries()) {
      const pkg = await Package.load(name, pkgInfo);
      const changes = getChangesFromGit(repo, pkg.name, pkg.version, pkg.scopes);
      if (changes != null) {
        const rule = calcBumpRuleFromChanges(pkg.version, changes);
        pkg.bumpVersion(rule);
      }
      const diff =
        pkg.versionDiff !== 0 ? `${pkg.version.format()} -> ${pkg.nextVersion.format()}` : pkg.version.format();
      this.context.stdout.write(`Changes for "${pkg.name}": ${diff}\n`);
    }
    // const actions: Action[] = [];
    // const versionedFile = await loadVersionedFile(path.join(rootDir, 'packages/cli/package.json'));
    // versionedFile.bumpVersion({ type: 'minor' });
    // const action = versionedFile.write();
    // if (action != null) {
    //   actions.push(action);
    // }
    // for (const action of actions) {
    //   this.context.stdout.write(formatAction(action));
    //   this.context.stdout.write('\n');
    // }
    return 0;
  }
}
