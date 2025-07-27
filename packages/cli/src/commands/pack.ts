import fs from 'node:fs/promises';
import path from 'node:path';
import { type BundleFile, create, encode } from '@webview-bundle/node-binding';
import { Option } from 'clipanion';
import { glob } from 'tinyglobby';
import { colors } from '../console.js';
import { formatByteLength } from '../utils/format.js';
import { BaseCommand } from './base.js';

export class PackCommand extends BaseCommand {
  readonly name = 'pack';
  static paths = [['pack']];

  readonly dir = Option.String({
    name: 'DIRECTORY',
    required: true,
  });
  readonly outfile = Option.String('--outfile,-O', {
    description: `Outfile path to create webview bundle archive.
If not provided, will create file with directory name.
If extension not set, will automatically add extension (\`.wvb\`)`,
    required: false,
  });
  readonly ignores = Option.Array('--ignore', [], {
    description: 'Ignore patterns.',
  });
  readonly truncate = Option.Boolean('--truncate', false, {
    description: 'Truncate outfile if file is already exists. Default: false',
  });
  readonly dryRun = Option.Boolean('--dry-run', false, {
    description: "Don't create webview bundle file on disk, instead just simulate packing files. Default: false",
  });

  async run() {
    const dir = path.isAbsolute(this.dir) ? this.dir : path.join(process.cwd(), this.dir);
    const pattern = path.join(dir, '**/*');
    const files = await glob([pattern], {
      cwd: dir,
      onlyFiles: true,
      dot: true,
      ignore: this.ignores,
    });
    if (files.length === 0) {
      this.logger.warn('No files to pack.');
      return 1;
    }
    this.logger.info(`Target ${files.length} files:`);
    const bundleFiles: BundleFile[] = [];
    for (const file of files) {
      const filepath = path.join(dir, file);
      const data = await fs.readFile(filepath);
      this.logger.info(`- ${colors.info(file)} ${colors.bytes(formatByteLength(data.byteLength))}`);
      bundleFiles.push({ path: file, data });
    }
    let outfile = this.outfile ?? path.basename(this.dir);
    if (path.extname(outfile) === '') {
      outfile = `${outfile}.wvb`;
    }
    const outfilePath = path.isAbsolute(outfile) ? outfile : path.join(process.cwd(), outfile);
    const bundle = await create(bundleFiles);
    if (!this.dryRun && !this.truncate) {
      const exists = await fs
        .access(outfilePath)
        .then(() => true)
        .catch(() => false);
      if (exists) {
        this.logger.error(`Outfile already exists: ${colors.bold(colors.error(outfile))}`);
        return 1;
      }
    }
    const bundleData = await encode(bundle);
    this.logger.info(
      `Output: ${colors.bold(colors.success(outfile))} ${colors.bytes(formatByteLength(bundleData.byteLength))}`
    );
    if (this.dryRun) {
      this.logger.debug(`Skip for write files on disk, because it's dry run.`);
    } else {
      await fs.writeFile(outfilePath, bundleData);
    }
  }
}
