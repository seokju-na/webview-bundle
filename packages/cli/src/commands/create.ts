import fs from 'node:fs/promises';
import path from 'node:path';
import type { HeadersConfig, IgnoreConfig } from '@webview-bundle/config';
import { BundleBuilder, writeBundle } from '@webview-bundle/node';
import { Command, Option } from 'clipanion';
import { filterAsync } from 'es-toolkit';
import pm from 'picomatch';
import { glob } from 'tinyglobby';
import { resolveConfig } from '../config.js';
import { c } from '../console.js';
import { formatByteLength } from '../format.js';
import { toAbsolutePath } from '../fs.js';
import { isLogLevelAtLeast } from '../log.js';
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

  readonly dir = Option.String({ name: 'DIR', required: false });
  readonly outfile = Option.String('--outfile,-O', {
    description: `Outfile path to create webview bundle archive.
If not provided, will create file with directory name.
If extension not set, will automatically add extension (\`.wvb\`)`,
    required: false,
  });
  readonly ignores = Option.Array('--ignore', {
    description: 'Ignore patterns.',
  });
  readonly truncate = Option.Boolean('--truncate', false, {
    description: 'Truncate outfile if file is already exists. [Default: false]',
  });
  readonly dryRun = Option.Boolean('--dry-run', false, {
    description: "Don't create webview bundle file on disk, instead just simulate packing files. [Default: false]",
  });
  readonly headers = Option.Array('--header,-H', {
    description:
      "Headers to set for each file. For example, `--header '*.html' 'cache-control' 'max-age=3600'` will set `cache-control: max-age=3600` for all files with extension `.html`.",
    arity: 3,
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
    const dirInput = this.dir ?? config.create?.srcDir;
    if (dirInput == null) {
      this.logger.error(
        'Source directory is not specified. Set "create.srcDir" in the config file or pass <DIR> as a CLI argument.'
      );
      return 1;
    }
    const dir = toAbsolutePath(dirInput, this.cwd);
    const allFiles = await glob('**/*', {
      cwd: dir,
      onlyFiles: true,
      dot: true,
      debug: isLogLevelAtLeast(this.logLevel, 'debug'),
    });
    const files = await filterAsync(allFiles, async file => {
      const ignores = [this.ignores, config.create?.ignore].filter(x => x != null);
      if (ignores.length === 0) {
        return true;
      }
      const ignored = await this.isMatchIgnores(file, ignores);
      return !ignored;
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

      const headersInput = [
        config.create?.headers,
        this.headers != null ? this.intoHeaderConfig(this.headers) : undefined,
      ].filter(x => x != null);
      const headers = await this.getHeaders(file, headersInput);

      builder.insertEntry(`/${file}`, data, undefined, headers);
    }
    let outfile = this.outfile ?? config.create?.outFile ?? path.basename(dir);
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

  private intoHeaderConfig(headers: [string, string, string][]): HeadersConfig {
    const config: Record<string, [string, string][]> = {};
    for (const [pattern, key, value] of headers) {
      if (config[pattern] == null) {
        config[pattern] = [[key, value]];
      } else {
        config[pattern]!.push([key, value]);
      }
    }
    return config;
  }

  private async getHeaders(file: string, headerInputs: HeadersConfig[]): Promise<Record<string, string> | undefined> {
    if (headerInputs.length === 0) {
      return undefined;
    }
    let headers = new Headers();
    for (const headerInput of headerInputs) {
      if (typeof headerInput === 'function') {
        const init = await headerInput(file);
        if (init != null) {
          headers = new Headers(init);
        }
      }
      const normalizedInput = Array.isArray(headerInput)
        ? headerInput
        : typeof headerInput === 'object' && headerInput != null
          ? Object.entries(headerInput)
          : [];
      for (const [pattern, init] of normalizedInput) {
        if (pm.isMatch(file, pattern)) {
          const appendHeaders = new Headers(init);
          for (const [key, value] of appendHeaders.entries()) {
            headers.set(key, value);
          }
        }
      }
    }
    const entries = [...headers.entries()];
    if (entries.length === 0) {
      return undefined;
    }
    return Object.fromEntries(entries);
  }

  private async isMatchIgnores(file: string, ignoreInputs: IgnoreConfig[]): Promise<boolean> {
    for (const ignoreInput of ignoreInputs) {
      if (typeof ignoreInput === 'function') {
        return ignoreInput(file);
      }
      if (Array.isArray(ignoreInputs)) {
        for (const ignore of ignoreInput) {
          if (typeof ignore === 'string') {
            if (pm.isMatch(file, ignore)) {
              return true;
            }
          } else {
            if (ignore.test(file)) {
              return true;
            }
          }
        }
      }
    }
    return false;
  }
}
