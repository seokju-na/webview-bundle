import path from 'node:path';
import { readBundle, writeBundleIntoBuffer } from '@wvb/node';
import { Command, Option } from 'clipanion';
import { isBoolean } from 'typanion';
import { defaultOutDir, defaultOutFile, resolveConfig } from '../../config.js';
import { c } from '../../console.js';
import { formatByteLength } from '../../format.js';
import { toAbsolutePath } from '../../fs.js';
import { buildURL } from '../../utils/url.js';
import { BaseCommand } from '../base.js';

export class RemoteUploadCommand extends BaseCommand {
  readonly name = 'remote-upload';

  static paths = [['remote', 'upload']];
  static usage = Command.Usage({
    description: 'Upload webview bundle to remote server.',
  });

  readonly bundleName = Option.String({
    name: 'BUNDLE',
    required: false,
  });
  readonly file = Option.String('--file,-F', {
    description: '',
  });
  readonly version = Option.String('--version,-V', {
    description: `Version of Webview Bundle to upload.
If not provided, default to version field in "package.json".`,
  });
  readonly force = Option.String('--force', {
    tolerateBoolean: true,
    validator: isBoolean(),
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
      this.logger.error(
        'Cannot get "uploader" from remote config. Make sure the "uploader" is defined in remote config.'
      );
      return 1;
    }
    const defaultFile = defaultOutFile(config);
    const fileInput = this.file ?? (defaultFile != null ? path.join(defaultOutDir(config), defaultFile) : undefined);
    if (fileInput == null) {
      this.logger.error(
        'Webview Bundle file is not specified. Set "outFile" in the config file ' +
          'or pass "--file,-F" as a CLI argument.'
      );
      return 1;
    }
    const filepath = toAbsolutePath(fileInput, config.root);
    const bundle = await readBundle(filepath);
    const bundleName = this.bundleName ?? config.remote?.bundleName ?? path.basename(filepath, '.wvb');
    const version = this.version ?? config.packageJson?.version;
    if (version == null) {
      this.logger.error('Cannot get version of this Webview Bundle.');
      return 1;
    }
    await config.remote.uploader.upload(
      {
        bundle,
        bundleName,
        version,
        force: this.force ?? false,
      },
      config.remote
    );
    const buf = writeBundleIntoBuffer(bundle);
    const size = buf.byteLength;
    const dest =
      config.remote.endpoint != null
        ? buildURL(config.remote.endpoint, `/bundles/${bundleName}`).toString()
        : bundleName;
    this.logger.info(`Webview Bundle uploaded: ${c.info(dest)} ${c.bytes(formatByteLength(size))}`);
  }
}
