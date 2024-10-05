import { Command } from 'clipanion';
import * as fsUtils from '../fsUtils';
import * as gitUtils from '../gitUtils';

export class ReleaseCommand extends Command {
  static paths = [['release']];

  async execute() {
    const rootDir = fsUtils.findRootDir();
    const repo = gitUtils.openRepo(rootDir);
    const head = repo.head();
    const commits = gitUtils.listCommits(repo, head.target()!);
    console.log(commits);
    return 0;
  }
}
