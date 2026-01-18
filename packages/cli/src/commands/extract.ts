import fs from 'node:fs/promises';
import path from 'node:path';
import { readBundle } from '@webview-bundle/node';
import { Command, Option } from 'clipanion';
import { isBoolean } from 'typanion';
import { defaultOutFile, resolveConfig } from '../config.js';
import { c } from '../console.js';
import { formatByteLength } from '../format.js';
import { pathExists, toAbsolutePath, withWVBExtension } from '../fs.js';
import { BaseCommand } from './base.js';

export class ExtractCommand extends BaseCommand {
  readonly name = 'extract';
  static paths = [['extract']];
  static usage = Command.Usage({
    description: 'Extract webview bundle files.',
    examples: [
      ['A basic usage', '$0 extract ./dist.wvb'],
      ['Specify outdir path', '$0 extract ./dist.wvb --outdir ./dist'],
    ],
  });

  readonly file = Option.String({
    name: 'FILE',
    required: false,
  });
  readonly outDir = Option.String('--outdir,-O', {
    description: `Outdir path to extract webview bundle files.
If not provided, will use webview bundle file name as directory.`,
  });
  readonly dryRun = Option.String('--dry-run', {
    tolerateBoolean: true,
    validator: isBoolean(),
    description:
      "Don't create extract files on disk, instead just look what inside on the webview bundle file. [Default: false]",
  });
  readonly clean = Option.String('--clean', {
    tolerateBoolean: true,
    validator: isBoolean(),
    description: 'Clean up extracted files if out directory already exists. [Default: false]',
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
    const file = this.file ?? config.extract?.file ?? defaultOutFile(config);
    if (file == null) {
      this.logger.error(
        'Webview Bundle file is not specified. Set "extract.file" in the config file ' +
          'or pass [FILE] as a CLI argument.'
      );
      return 1;
    }
    const filepath = toAbsolutePath(withWVBExtension(file), config.root);
    if (!(await pathExists(filepath))) {
      this.logger.error(`File not found: ${filepath}`);
      return 1;
    }
    const bundle = await readBundle(filepath);
    this.logger.info(`Webview Bundle info for ${c.info(filepath)}`);
    this.logger.info(`Version: ${c.bold(c.info(bundle.descriptor().header().version()))}`);
    this.logger.info(`Entries:`);
    const entries = Object.entries(bundle.descriptor().index().entries());
    entries.sort((a, b) => {
      if (a[0] < b[0]) {
        return -1;
      } else if (a[0] > b[0]) {
        return 1;
      }
      return 0;
    });
    for (const [p, entry] of entries) {
      const data = bundle.getData(p)!;
      const bytes = formatByteLength(data.byteLength);
      this.logger.info(`${c.info(p)} ${c.bytes(bytes)}`);
      this.logger.info(`  ${c.header(['content-type', entry.contentType])}`);
      this.logger.info(`  ${c.header(['content-length', String(entry.contentLength)])}`);
      for (const h of Object.entries(entry.headers)) {
        this.logger.info(`  ${c.header(h)}`);
      }
    }
    const dryRun = this.dryRun ?? config.extract?.dryRun ?? false;
    if (dryRun) {
      this.logger.info(`Skip writing files on disk, because it's dry run.`);
      return 0;
    }
    const outDir = this.outDir ?? config.extract?.outDir ?? path.basename(filepath, '.wvb');
    const outDirPath = toAbsolutePath(outDir, config.root);
    if (await pathExists(outDirPath)) {
      const clean = this.clean ?? config.extract?.clean ?? false;
      if (clean) {
        await fs.rm(outDirPath, { recursive: true });
      } else {
        this.logger.error(`Outdir already exists: ${outDirPath}`);
        return 1;
      }
    }
    const entryPaths = Object.keys(bundle.descriptor().index().entries());
    for (const p of entryPaths) {
      const filepath = path.join(outDirPath, p);
      await fs.mkdir(path.dirname(filepath), { recursive: true });
      await fs.writeFile(filepath, bundle.getData(p)!);
    }
    this.logger.info(`Extract completed: ${c.bold(c.success(outDirPath))}`);
    return 0;
  }
}
