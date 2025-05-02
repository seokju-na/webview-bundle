import path from 'node:path';
import type { Repository } from '@napi-rs/simple-git';
import chalk from 'chalk';
import { Command, Option } from 'clipanion';
import * as gitUtils from '../gitUtils';
import { getGithub } from '../github';
import * as workspaceUtils from '../workspaceUtils';
import { type Action, formatAction, runAction } from './action';
import { Changelog } from './changelog';
import { ReleaseConfig } from './config';
import { Package } from './package';

interface StepResult {
  name: string;
  package: Package;
  succeed: boolean;
}

export class ReleaseCommand extends Command {
  static paths = [['release']];

  dryRun = Option.Boolean('--dry-run', false, {
    description: "Perform dry run. Don't change any files or any network calls.",
  });

  githubToken = Option.String('--github-token', {
    required: true,
    env: 'GITHUB_TOKEN',
    description: 'GitHub token to use. Used to create GitHub Release.',
  });

  async execute() {
    const rootDir = workspaceUtils.findRootDir();
    const config = await ReleaseConfig.load(path.join(rootDir, 'releases.json'));

    const repo = gitUtils.openRepo(rootDir);
    const headRef = repo.head();
    const headCommit = repo.findCommit(headRef.target()!)!;
    const targetTags = gitUtils.listTags(repo).filter(x => x.oid === headCommit.id());

    const packages = await Package.loadAllFromConfig(config);
    const targetPackages = packages.filter(pkg => {
      const isInTarget = targetTags.some(x => x.name === pkg.versionGitTag);
      return isInTarget;
    });
    if (targetPackages.length === 0) {
      this.context.stdout.write(chalk.red('No packages to release'));
      this.context.stdout.write('\n');
      return 1;
    }
    const results = new Map<string, StepResult[]>();
    for (const pkg of targetPackages) {
      const stepResults = await this.runSteps(pkg, [
        { name: 'Run before release scripts', fn: () => this.runBeforeReleaseScripts(pkg, config) },
        { name: 'Publish', fn: () => this.publishPackage(pkg) },
        { name: 'Add git tag', fn: () => this.addGitTag(pkg, config, repo) },
        { name: 'Create GitHub release', fn: () => this.createGitHubRelease(pkg, config) },
      ]);
      results.set(`${pkg.name} v${pkg.version.format()}`, stepResults);
    }
    this.context.stdout.write('\n\n');
    for (const [name, stepResults] of results.entries()) {
      this.context.stdout.write(chalk.bold(name));
      for (const stepResult of stepResults) {
        this.context.stdout.write(
          stepResult.succeed ? `  ✅ ${stepResult.name} succeed` : `  ❌ ${stepResult.name} failed`
        );
        this.context.stdout.write('\n');
      }
      this.context.stdout.write('\n');
    }
    const allSucceed = [...results.values()].flat().every(x => x.succeed);
    return allSucceed ? 0 : 1;
  }

  async runSteps(pkg: Package, steps: Array<{ name: string; fn: () => Promise<unknown> }>): Promise<StepResult[]> {
    const results: StepResult[] = [];
    for (const step of steps) {
      try {
        await step.fn();
        results.push({ name: step.name, package: pkg, succeed: true });
      } catch (e) {
        this.context.stderr.write((e as Error)?.message);
        this.context.stderr.write('\n\n');
        results.push({ name: step.name, package: pkg, succeed: false });
        break;
      }
    }
    return results;
  }

  async runBeforeReleaseScripts(pkg: Package, config: ReleaseConfig) {
    const scripts = config.packages.get(pkg.name)?.beforeReleaseScripts ?? [];
    if (scripts.length === 0) {
      return;
    }
    for (const script of scripts) {
      const action: Action = {
        type: 'runCommand',
        command: script.command,
        cwd: script.cwd != null ? workspaceUtils.absolutePathFromRootDir(script.cwd) : workspaceUtils.findRootDir(),
        env: script.env,
      };
      this.context.stdout.write(formatAction(action));
      this.context.stdout.write('\n\n');
      if (!this.dryRun) {
        await runAction(action);
      }
    }
  }

  async addGitTag(pkg: Package, config: ReleaseConfig, repo: Repository) {
    const tagName = gitUtils.tagShorthand(pkg.nextVersionGitTag);
    const oid = repo.head().target()!;
    this.context.stdout.write(`Create Git tag: ${tagName} (${oid.slice(0, 7)})`);
    this.context.stdout.write('\n\n');
    if (this.dryRun) {
      return;
    }
    const github = getGithub(this.githubToken);
    const signature = gitUtils.defaultSignature();
    const { data: tag } = await github.git.createTag({
      owner: config.github.repo.owner,
      repo: config.github.repo.name,
      tag: tagName,
      message: tagName,
      type: 'commit',
      object: oid,
      tagger: {
        name: signature.name()!,
        email: signature.email()!,
      },
    });
    return tag;
  }

  async publishPackage(pkg: Package) {
    this.context.stdout.write(`Start publish for "${pkg.name}v${pkg.version.format()}"`);
    this.context.stdout.write('\n');
    for (const versionedFile of pkg.versionedFiles) {
      this.context.stdout.write(`Publish "${versionedFile.filepath}"\n`);
      const published = await versionedFile.publish(this.dryRun);
      if (!published) {
        this.context.stdout.write('=> Since it is not a publishable package, publishing is skipped');
      }
    }
  }

  async createGitHubRelease(pkg: Package, config: ReleaseConfig) {
    const title = `${pkg.name} v${pkg.version.format()}`;
    this.context.stdout.write(`Create GitHub release: ${title}`);
    this.context.stdout.write('\n');
    if (this.dryRun) {
      return;
    }
    const github = getGithub(this.githubToken);
    const changelog = await Changelog.load(config.packages.get(pkg.name)!.changelog);
    const { data: release } = await github.repos.createRelease({
      owner: config.github.repo.owner,
      repo: config.github.repo.name,
      name: title,
      tag_name: gitUtils.tagShorthand(pkg.versionGitTag),
      body: changelog.extractSection(title),
      prerelease: pkg.version.getPrereleaseIdentifier() != null,
      draft: false,
    });
    return release;
  }
}
