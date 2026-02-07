import fs from 'node:fs/promises';
import path from 'node:path';
import { type BundleManifestData, Remote } from '@wvb/node';
import { Command, Option } from 'clipanion';
import { isBoolean } from 'typanion';
import { resolveConfig } from '../config.js';
import { BaseCommand } from './base.js';

export class BuiltinCommand extends BaseCommand {
  readonly name = 'builtin';

  static paths = [['builtin']];
  static usage = Command.Usage({});

  readonly endpoint = Option.String('--endpoint,-E', {
    description: 'Endpoint of remote server.',
  });
  readonly clean = Option.String('--clean', true, {
    tolerateBoolean: true,
    validator: isBoolean(),
  });
  readonly configFile = Option.String('--config,-C', {
    description: 'Path to the config file.',
  });
  readonly cwd = Option.String('--cwd', {
    description: 'Set the working directory for resolving paths. [Default: process.cwd()]',
  });

  async run() {
    const config = await resolveConfig({
      root: this.cwd,
      configFile: this.configFile,
    });
    const manifest: BundleManifestData = {
      manifestVersion: 1,
      entries: {},
    };
    if (this.clean) {
      await fs.rm('bundles', { recursive: true });
    }
    const remote = new Remote('');
    const remoteBundles = await remote.listBundles();
    for (const remoteBundle of remoteBundles) {
      const [info, , buffer] = await remote.download(remoteBundle.name);
      manifest.entries[info.name] = {
        versions: {
          [info.version]: {
            etag: info.etag,
            integrity: info.integrity,
            signature: info.signature,
            lastModified: info.lastModified,
          },
        },
        currentVersion: info.version,
      };
      const filename = `${info.name}_${info.version}.wvb`;
      const filepath = path.join('bundles', info.name, filename);
      await fs.mkdir(path.dirname(filepath), { recursive: true });
      await fs.writeFile(filepath, buffer);
    }
  }
}
