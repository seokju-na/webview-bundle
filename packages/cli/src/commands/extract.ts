import { Command, Option } from 'clipanion';
import { isBoolean } from 'typanion';
import { defaultOutFile, resolveConfig } from '../config.js';
import { extract } from '../operations/extract.js';
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
    const file = this.file ?? config.extract?.file ?? defaultOutFile(config);
    if (file == null) {
      this.logger.error(
        'Webview Bundle file is not specified. Set "extract.file" in the config file ' +
          'or pass [FILE] as a CLI argument.'
      );
      return 1;
    }
    const dryRun = this.dryRun ?? config.extract?.dryRun ?? false;
    const outDir = this.outDir ?? config.extract?.outDir;
    const clean = this.clean ?? config.extract?.clean ?? false;
    await extract({
      file,
      outDir,
      cwd: config.root,
      write: !dryRun,
      clean,
      logger: this.logger,
    });
  }
}
