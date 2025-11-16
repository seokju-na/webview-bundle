import fs from 'node:fs/promises';
import path from 'node:path';
import { BundleBuilder, writeBundle } from '@webview-bundle/node';
import { Command, Option } from 'clipanion';
import pm from 'picomatch';
import { glob } from 'tinyglobby';
import { c } from '../console.js';
import { formatByteLength } from '../format.js';
import { BaseCommand } from './base.js';

export class CreateCommand extends BaseCommand {
  readonly name = 'create';
  static paths = [['create']];
  static usage = Command.Usage({
    description: 'Create webview bundle archive.',
    examples: [
      ['A basic usage', '$0 create ./dist'],
      ['Specify outfile path', '$0 create ./dist --outfile ./dist.wvb'],
      ['Ignore files with patterns', `$0 create ./dist --ignore='*.txt' --ignore='node_modules/**'`],
      ['Set headers for files', `$0 create ./dist --header='*.html' 'cache-control' 'max-age=3600'`],
    ],
  });

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
    description: 'Truncate outfile if file is already exists. [Default: false]',
  });
  readonly dryRun = Option.Boolean('--dry-run', false, {
    description: "Don't create webview bundle file on disk, instead just simulate packing files. [Default: false]",
  });
  readonly headers = Option.Array('--header,-H', [], {
    description:
      "Headers to set for each file. For example, `--header '*.html' 'cache-control' 'max-age=3600'` will set `cache-control: max-age=3600` for all files with extension `.html`.",
    arity: 3,
  });

  async run() {
    const dir = path.isAbsolute(this.dir) ? this.dir : path.join(process.cwd(), this.dir);
    const files = await glob('**/*', {
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
    const builder = new BundleBuilder();
    for (const file of files) {
      const filepath = path.join(dir, file);
      const data = await fs.readFile(filepath);
      this.logger.info(`- ${c.info(file)} ${c.bytes(formatByteLength(data.byteLength))}`);
      builder.insertEntry(`/${file}`, data, this.getHeaders(`/${file}`));
    }
    let outfile = this.outfile ?? path.basename(this.dir);
    if (path.extname(outfile) === '') {
      outfile = `${outfile}.wvb`;
    }
    const outfilePath = path.isAbsolute(outfile) ? outfile : path.join(process.cwd(), outfile);
    const bundle = builder.build();
    if (this.dryRun) {
      this.logger.debug(`Skip for write files on disk, because it's dry run.`);
      return 0;
    }
    if (!this.truncate) {
      const exists = await fs
        .access(outfilePath)
        .then(() => true)
        .catch(() => false);
      if (exists) {
        this.logger.error(`Outfile already exists: ${c.bold(c.error(outfile))}`);
        return 1;
      }
    }
    await fs.mkdir(path.dirname(outfilePath), { recursive: true });
    const size = await writeBundle(bundle, outfilePath);
    this.logger.info(`Output: ${c.bold(c.success(outfile))} ${c.bytes(formatByteLength(Number(size)))}`);
  }

  private getHeaders(file: string): Record<string, string> | undefined {
    let isEmpty = true;
    const value: Record<string, string> = {};
    for (const header of this.headers) {
      const [pattern, headerName, headerValue] = header;
      if (pm.isMatch(file, pattern)) {
        isEmpty = false;
        value[headerName] = headerValue;
      }
    }
    if (isEmpty) {
      return undefined;
    }
    return value;
  }
}
