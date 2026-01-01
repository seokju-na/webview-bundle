import path from 'node:path';
import { readBundle } from '@webview-bundle/node';
import { Command, Option } from 'clipanion';
import { resolveConfig } from '../../config';
import { BaseCommand } from '../base';

export class RemoteUploadCommand extends BaseCommand {
  readonly name = 'remote-upload';

  static paths = [['remote', 'upload']];
  static usage = Command.Usage({
    description: 'Upload webview bundle to remote server.',
  });

  readonly file = Option.String({
    name: 'FILE',
    required: true,
  });
  readonly version = Option.String({
    name: 'VERSION',
    required: true,
  });
  readonly bundleName = Option.String('--name,-N', {
    description: 'Bundle name to upload. Default to file name.',
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
    if (config.remote?.uploader == null) {
      throw new Error('no remote config found');
    }
    const bundleFilepath = path.isAbsolute(this.file) ? this.file : path.join(config.root, this.file);
    const bundle = await readBundle(bundleFilepath);
    let bundleName = this.bundleName ?? path.basename(bundleFilepath);
    if (bundleName.endsWith('.wvb')) {
      bundleName = bundleName.replace(/\.wvb$/, '');
    }
    await config.remote.uploader.upload(bundleName, this.version, bundle);
  }
}
