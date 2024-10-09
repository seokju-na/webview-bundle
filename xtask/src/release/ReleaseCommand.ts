import path from 'node:path';
import chalk from 'chalk';
import { Command, Option } from 'clipanion';
import * as gitUtils from '../gitUtils';
import { getGithub } from '../github';
import * as workspaceUtils from '../workspaceUtils';
import { Changelog } from './changelog';
import { ReleaseConfig } from './config';
import { Package } from './package';

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
    let exitCode = 0;
    const results: string[] = [];
    for (const pkg of targetPackages) {
      try {
        await this.publishPackage(pkg);
        results.push(`  ✅ Publish succeed: "${pkg.name}v${pkg.version.format()}"`);
      } catch (e) {
        this.context.stderr.write((e as Error)?.message);
        this.context.stderr.write('\n');
        exitCode = 1;
        results.push(`  ❌ Publish failed: "${pkg.name}v${pkg.version.format()}"`);
        continue;
      }

      try {
        const release = await this.createGitHubRelease(pkg, config);
        results.push(`  ✅ Create GitHub release succeed: "${pkg.name}v${pkg.version.format()}" (${release.html_url})`);
      } catch (e) {
        this.context.stderr.write((e as Error)?.message);
        this.context.stderr.write('\n');
        exitCode = 1;
        results.push(`  ❌ Create GitHub release failed: "${pkg.name}v${pkg.version.format()}"`);
      }
    }
    this.context.stdout.write('\n\nRelease results:');
    for (const result of results) {
      this.context.stdout.write(result);
      this.context.stdout.write('\n');
    }
    return exitCode;
  }

  async publishPackage(pkg: Package) {
    this.context.stdout.write(`Start publish "${pkg.name}v${pkg.version.format()}"`);
    this.context.stdout.write('\n');
    // Run publish command for each versioned file
    for (const versionedFile of pkg.versionedFiles) {
      this.context.stdout.write(`Publish "${versionedFile.filepath}"\n`);
      const published = await versionedFile.publish(this.dryRun);
      if (!published) {
        this.context.stdout.write('=> Since it is not a publishable package, publishing is skipped');
      }
    }
  }

  async createGitHubRelease(pkg: Package, config: ReleaseConfig) {
    const github = getGithub(this.githubToken);
    const changelog = await Changelog.load(config.packages.get(pkg.name)!.changelog);
    const title = `${pkg.name} v${pkg.version.format()}`;
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
