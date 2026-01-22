import { Command, Option } from 'clipanion';
import { BaseCommand } from '../base.js';

export class RemoteDeployCommand extends BaseCommand {
  readonly name = 'remote-deploy';

  static paths = [['remote', 'deploy']];
  static usage = Command.Usage({
    description: 'Deploy Webview Bundle in remote server.',
  });

  readonly bundleName = Option.String();
  readonly version = Option.String('--version,-V', {
    description: `Version of Webview Bundle to upload.
If not provided, default to version field in "package.json".`,
  });

  async run() {
    throw new Error('TODO');
  }
}
