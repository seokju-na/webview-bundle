import { BaseCommand } from '../base.js';

export class RemoteDeployCommand extends BaseCommand {
  readonly name = 'remote-deploy';

  static paths = [['remote', 'deploy']];

  async run() {}
}
