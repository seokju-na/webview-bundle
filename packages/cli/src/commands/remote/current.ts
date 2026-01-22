import { Remote } from '@webview-bundle/node';
import { Command, Option } from 'clipanion';
import { resolveConfig } from '../../config.js';
import { c } from '../../console.js';
import { BaseCommand } from '../base.js';

export class RemoteCurrentCommand extends BaseCommand {
  readonly name = 'remote-current';

  static paths = [['remote', 'current']];
  static usage = Command.Usage({
    description: 'Show current Webview Bundle information from remote server.',
    examples: [['A basic usage', '$0 remote current --name my-bundle --endpoint https://my-remote-server.com']],
  });

  readonly bundleName = Option.String({
    name: 'BUNDLE',
    required: false,
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
    const endpoint = this.endpoint ?? config.remote?.endpoint;
    if (endpoint == null) {
      this.logger.error('"endpoint" is required for remote operations.');
      return 1;
    }
    const bundleName = this.bundleName ?? config.remote?.bundleName;
    if (bundleName == null) {
      this.logger.error('"bundleName" is required for remote operations.');
      return 1;
    }
    const remote = new Remote(endpoint, {
      http: config.remote?.http,
    });
    const info = await remote.getInfo(bundleName);
    this.logger.info(`Remote Webview Bundle info for ${c.info(bundleName)}`);
    this.logger.info(`  Version: ${c.bold(c.info(info.version))}`);
    this.logger.info(`  ETag: ${c.bold(c.info(info.etag ?? '(none)'))}`);
    this.logger.info(`  Integrity: ${c.bold(c.info(info.integrity ?? '(none)'))}`);
    this.logger.info(`  Signature: ${c.bold(c.info(info.signature ?? '(none)'))}`);
    this.logger.info(`  Last-Modified: ${c.bold(c.info(info.lastModified ?? '(none)'))}`);
  }
}
