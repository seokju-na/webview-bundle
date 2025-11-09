import { checkbox, select } from '@inquirer/prompts';
import { Command, Option } from 'clipanion';
import { openRepository } from 'es-git';
import { Changes } from '../changes.ts';
import { loadConfig } from '../config.ts';
import { ColorModeOption, colors, setColorMode } from '../console.ts';
import { ROOT_DIR } from '../consts.ts';
import { Package } from '../package.ts';
import { loadStaged, type Staged, saveStaged } from '../staged.ts';

export class PrepareReleaseCommand extends Command {
  static paths = [['prepare-release']];

  readonly configFilepath = Option.String('--config', 'xtask.json');
  readonly dryRun = Option.Boolean('--dry-run', false);
  readonly stagedFilepath = Option.String('--staged', 'staged.json');
  readonly colorMode = ColorModeOption;

  async execute() {
    setColorMode(this.colorMode);
    const config = await loadConfig(this.configFilepath);
    const staged = await loadStaged(this.stagedFilepath).catch(() => ({}) as Staged);
    const repo = await openRepository(ROOT_DIR);
    const head = repo.head().target();
    if (head == null) {
      throw new Error('cannot find git `HEAD` target');
    }
    const packages = await Package.loadAll(config);
    for (const pkg of packages) {
      const tag = pkg.versionedGitTag.findTag(repo);
      const revwalk = repo.revwalk();
      if (tag != null) {
        revwalk.hide(tag.id());
      }
      const allCommits = [...revwalk].map(x => repo.getCommit(x));
      if (allCommits.length === 0) {
        console.log(`${colors.warn(`[${pkg.name}]`)} no commits found. skip.`);
        continue;
      }
      const commits = await checkbox({
        message: `${colors.info(`[${pkg.name}]`)} Select commits to include in release`,
        choices: allCommits.map(commit => {
          const checked = staged[pkg.name]?.commits.some(x => x === commit.id());
          return {
            name: `[${commit.id().slice(0, 7)}] ${commit.summary() ?? '(no message)'}`,
            value: commit.id(),
            checked,
          };
        }),
        loop: false,
      });
      const initialBumpRule = Changes.fromCommits(repo, commits).getBumpRule();
      if (initialBumpRule == null) {
        continue;
      }
      const bumpRule = await select({
        message: `${colors.info(`[${pkg.name}]`)} Select bump rule`,
        choices: (['major', 'minor', 'patch'] as const).map(rule => {
          const checked = rule === initialBumpRule?.type;
          return {
            name: rule,
            value: rule,
            checked,
          };
        }),
        loop: false,
      });
      staged[pkg.name] = {
        commits: commits,
        bumpRule: { type: bumpRule },
      };
    }
    if (this.dryRun) {
      console.log(JSON.stringify(staged, null, 2));
      return 0;
    }
    await saveStaged(this.stagedFilepath, staged);
  }
}
