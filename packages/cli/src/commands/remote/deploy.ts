import { Command, Option } from 'clipanion';
import { resolveConfig } from '../../config.js';
import { BaseCommand } from '../base.js';

export class RemoteDeployCommand extends BaseCommand {
  readonly name = 'remote-deploy';

  static paths = [['remote', 'deploy']];
  static usage = Command.Usage({
    description: 'Deploy Webview Bundle in remote server.',
  });

  readonly bundleName = Option.String({
    name: 'BUNDLE',
    required: false,
  });
  readonly version = Option.String('--version,-V', {
    description: `Version of Webview Bundle to upload.
If not provided, default to version field in "package.json".`,
  });
  readonly configFile = Option.String('--config,-C', {
    description: 'Config file path',
  });
  readonly cwd = Option.String('--cwd', {
    description: 'Current working directory.',
  });

  async run() {
    const config = await resolveConfig({
      root: this.cwd,
      configFile: this.configFile,
    });
    if (config.remote?.deployer == null) {
      this.logger.error(
        'Cannot get "deployer" from remote config. Make sure the "deployer" is defined in remote config.'
      );
      return 1;
    }
    throw new Error('TODO');
  }
}
