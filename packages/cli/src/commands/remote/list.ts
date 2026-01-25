import { Remote } from '@wvb/node';
import { Command, Option } from 'clipanion';
import { resolveConfig } from '../../config.js';
import { BaseCommand } from '../base.js';

export class RemoteListCommand extends BaseCommand {
  readonly name = 'remote-list';

  static paths = [
    ['remote', 'list'],
    ['remote', 'ls'],
  ];
  static usage = Command.Usage({
    description: '',
  });

  readonly endpoint = Option.String('--endpoint,-E', {
    description: 'Endpoint of remote server.',
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
    const endpointInput = this.endpoint ?? config.remote?.endpoint;
    if (endpointInput == null) {
      this.logger.error('"endpoint" is required for remote operations.');
      return 1;
    }
    const remote = new Remote(endpointInput);
    const bundles = await remote.listBundles();
    this.logger.info(`Remote Webview Bundles:`);
    console.log(JSON.stringify(bundles, null, 2));
  }
}
