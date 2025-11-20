import { resolveConfig } from '@webview-bundle/config';
import { Command, Option } from 'clipanion';

export class RemoteUploadCommand extends Command {
  static paths = [['remote', 'upload']];

  readonly cwd = Option.String('--cwd', {
    description: 'Current working directory.',
  });

  async execute() {
    const config = await resolveConfig({
      root: this.cwd,
    });
    if (config.remote?.uploader == null) {
      throw new Error('no remote config');
    }
    await config.remote.uploader.upload('', '', null as any);
  }
}
