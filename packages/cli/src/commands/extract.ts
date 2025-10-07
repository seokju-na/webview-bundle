import fs from 'node:fs/promises';
import path from 'node:path';
import { readBundle } from '@webview-bundle/node';
import { Command, Option } from 'clipanion';
import { c } from '../console.js';
import { formatByteLength } from '../utils/format.js';
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
    required: true,
  });
  readonly outdir = Option.String('--outdir,-O', {
    description: `Outdir path to extract webview bundle files.
If not provided, will use webview bundle file name as directory.`,
  });
  readonly dryRun = Option.Boolean('--dry-run', false, {
    description:
      "Don't create extract files on disk, instead just look what inside on the webview bundle file. Default :false",
  });

  async run() {
    const filepath = path.isAbsolute(this.file) ? this.file : path.join(process.cwd(), this.file);
    const bundle = await readBundle(filepath);
    this.logger.info(`Webview Bundle info for ${c.info(filepath)}`);
    this.logger.info(`Version: ${c.bold(c.info(bundle.manifest().header().version()))}`);
    this.logger.info(`Entries:`);
    const entries = Object.entries(bundle.manifest().index().entries());
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
      for (const h of Object.entries(entry.headers)) {
        this.logger.info(`  ${c.header(h)}`);
      }
    }
    if (this.dryRun) {
      this.logger.debug(`Skip for write files on disk, because it's dry run.`);
      return 0;
    }
    let outdir = this.outdir ?? path.basename(filepath);
    if (path.extname(outdir) === '.wvb') {
      outdir = outdir.replace(/\.wvb$/, '');
    }
    const outdirPath = path.isAbsolute(outdir) ? outdir : path.join(process.cwd(), outdir);
    const exists = await fs
      .access(outdirPath)
      .then(() => true)
      .catch(() => false);
    if (exists) {
      this.logger.error(`Outdir already exists: ${outdirPath}`);
      return 1;
    }
    const entryPaths = Object.keys(bundle.manifest().index().entries());
    for (const p of entryPaths) {
      const filepath = path.join(outdirPath, p);
      await fs.mkdir(path.dirname(filepath), { recursive: true });
      await fs.writeFile(filepath, bundle.getData(p)!);
    }
    this.logger.info(`Extract completed: ${c.bold(c.success(outdirPath))}`);
    return 0;
  }
}
