import { Option } from 'clipanion';
import { BaseCommand } from '../base';

export class RemoteDeployCommand extends BaseCommand {
  readonly name = 'remote-deploy';

  readonly configFile = Option.String('--config,-C', {
    description: 'Config file path',
  });

  async run() {}
}
