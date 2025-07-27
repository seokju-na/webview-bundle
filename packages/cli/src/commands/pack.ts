import fs from 'node:fs/promises';
import path from 'node:path';
import { type BundleFile, create, encode } from '@webview-bundle/node-binding';
import { Command, Option } from 'clipanion';
import { glob } from 'tinyglobby';
import { applyColorOption, ColorOption } from '../console.js';
import { formatByteLength } from '../utils/format.js';

export class PackCommand extends Command {
  static paths = [['pack']];

  readonly color = ColorOption;
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
    description: 'Truncate outfile if file is already exists.',
  });

  async execute() {
    applyColorOption(this.color);
    const dir = path.isAbsolute(this.dir) ? this.dir : path.join(process.cwd(), this.dir);
    const pattern = path.join(dir, '**/*');
    const files = await glob([pattern], {
      cwd: dir,
      onlyFiles: true,
      dot: true,
      ignore: this.ignores,
    });
    if (files.length === 0) {
      console.log('No files to pack.');
      return 1;
    }
    console.log('Target files:');
    const bundleFiles: BundleFile[] = [];
    for (const file of files) {
      const filepath = path.join(dir, file);
      const data = await fs.readFile(filepath);
      console.log(`- ${file} (${formatByteLength(data.byteLength)})`);
      bundleFiles.push({ path: file, data });
    }
    let outfile = this.outfile ?? path.basename(this.dir);
    if (path.extname(outfile) === '') {
      outfile = `${outfile}.wvb`;
    }
    const outfilePath = path.join(process.cwd(), outfile);
    const bundle = await create(bundleFiles);
    if (!this.truncate) {
      const exists = await fs
        .access(outfilePath)
        .then(() => true)
        .catch(() => false);
      if (exists) {
        console.error(`Outfile already exists: ${outfile}`);
        return 1;
      }
    }
    const bundleData = await encode(bundle);
    console.log(`Output: ${outfile} (${formatByteLength(bundleData.byteLength)})`);
    await fs.writeFile(outfilePath, bundleData);
  }
}
