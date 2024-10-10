import path from 'node:path';
import { Command, Option } from 'clipanion';
import * as gitUtils from '../gitUtils';
import { getGithub } from '../github';
import * as workspaceUtils from '../workspaceUtils';
import { ReleaseConfig } from './config';

export class PrepareReleasePRCommand extends Command {
  static paths = [['prepare-release-pr']];

  base = Option.String('-b,--base', 'main', {
    description: 'Base branch for Pull Request.',
  });

  target = Option.String('-t,--target', {
    description: 'Target branch reference for Pull Request. Default to current branch.',
  });

  title = Option.String({ required: true, name: 'Title of pull request' });

  githubToken = Option.String('--github-token', {
    required: true,
    env: 'GITHUB_TOKEN',
    description: 'GitHub token to use. Used to create GitHub Release.',
  });

  async execute() {
    const rootDir = workspaceUtils.findRootDir();
    const config = await ReleaseConfig.load(path.join(rootDir, 'releases.json'));

    const repo = gitUtils.openRepo(rootDir);
    const base = this.base;
    const target = this.target ?? repo.head().target()!;

    const exists = await this.findPullRequest(config, base, target);
    if (exists != null) {
      this.context.stdout.write(`Pull Request already exists (${exists.html_url})\n`);
      this.context.stdout.write('Skip creating Pull Request.\n');
    } else {
      this.context.stdout.write(`Creating Pull Request (base=${base}, target=${target})\n`);
      const pr = await this.createPullRequest(
        config,
        base,
        target,
        this.title,
        'This PR was automatically generated for the release.'
      );
      this.context.stdout.write(`Pull Request created: ${pr.html_url}\n`);
    }
  }

  private async findPullRequest(config: ReleaseConfig, base: string, target: string) {
    const github = getGithub(this.githubToken);
    const { data: prs } = await github.pulls.list({
      owner: config.github.repo.owner,
      repo: config.github.repo.name,
      base,
      head: `${config.github.repo.owner}:${target}`,
    });
    return prs[0];
  }

  private async createPullRequest(config: ReleaseConfig, base: string, target: string, title: string, body: string) {
    const github = getGithub(this.githubToken);
    const { data: pr } = await github.pulls.create({
      owner: config.github.repo.owner,
      repo: config.github.repo.name,
      base,
      head: target,
      title,
      body,
    });
    return pr;
  }
}
