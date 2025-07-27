import fs from 'node:fs/promises';
import path from 'node:path';
import { decode } from '@webview-bundle/node-binding';
import { Option } from 'clipanion';
import { colors } from '../console.js';
import { formatByteLength } from '../utils/format.js';
import { BaseCommand } from './base.js';

export class ExtractCommand extends BaseCommand {
  readonly name = 'extract';
  static paths = [['extract']];

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
    const fileData = await fs.readFile(filepath);
    const bundle = await decode(fileData);
    const files = await bundle.readAllFiles();
    this.logger.info(`Webview Bundle info: ${colors.bold(colors.info(filepath))}`);
    this.logger.info(`Version: ${colors.bold(colors.info(bundle.version()))}`);
    this.logger.info(`Files:`);
    for (const file of files) {
      const bytes = formatByteLength(file.data.byteLength);
      this.logger.info(`- ${colors.info(file.path)} ${colors.bytes(bytes)}`);
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
    for (const file of files) {
      const filepath = path.join(outdirPath, file.path);
      await fs.mkdir(path.dirname(filepath), { recursive: true });
      await fs.writeFile(filepath, file.data);
    }
    this.logger.info(`Extract completed: ${colors.bold(colors.success(outdirPath))}`);
    return 0;
  }
}
