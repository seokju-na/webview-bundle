import fs from 'node:fs/promises';
import path from 'node:path';
import { Command, Option } from 'clipanion';
import { uniq } from 'es-toolkit';
import type { PackageJson } from 'type-fest';
import { buildDTS } from './buildDTS';
import { buildJSFiles } from './buildJSFiles';

export class BuildJSLibraryCommand extends Command {
  static paths = [['build-js-library']];

  src = Option.String({ name: 'Path of source files', required: true });
  cjsOutdir = Option.String('--cjs-outdir', 'dist', {
    description: 'Output directory for cjs format files.',
  });
  esmOutdir = Option.String('--esm-outdir', 'esm', {
    description: 'Output directory for esm format files.',
  });
  dtsOutdir = Option.String('--dts-outdir', 'dist', {
    description: 'Output directory for typescript definition files.',
  });
  platform = Option.String('--platform', {
    description: 'Which platform will library used.',
  });
  target = Option.Array('--target', {
    description: 'Target for library.',
  });

  async execute() {
    const cwd = process.cwd();
    const pkgJson = await this.readPackageJson();

    const distDir = path.join(cwd, this.cjsOutdir);
    const esmDir = path.join(cwd, this.esmOutdir);
    const dtsDir = path.join(cwd, this.dtsOutdir);

    this.context.stdout.write('Cleanup directories:\n');
    this.context.stdout.write(`  cjs: ${this.cjsOutdir}\n`);
    this.context.stdout.write(`  esm: ${this.esmOutdir}\n`);
    this.context.stdout.write(`  dts: ${this.dtsOutdir}\n`);
    await Promise.all([
      fs.rm(distDir, { force: true, recursive: true }),
      fs.rm(esmDir, { force: true, recursive: true }),
      fs.rm(dtsDir, { force: true, recursive: true }),
    ]);

    const external = uniq([
      ...Object.keys(pkgJson.peerDependencies ?? {}),
      ...Object.keys(pkgJson.dependencies ?? {}),
      ...Object.keys(pkgJson.devDependencies ?? {}),
    ]);
    await buildJSFiles(this.src, {
      cwd,
      outdir: {
        cjs: this.cjsOutdir,
        esm: this.esmOutdir,
      },
      target: this.target,
      platform: this.platform as 'browser' | 'node' | 'neutral',
      external,
      stdout: this.context.stdout,
      stderr: this.context.stderr,
    });
    await buildDTS(this.src, this.dtsOutdir, {
      cwd,
      stdout: this.context.stdout,
      stderr: this.context.stderr,
    });
  }

  private async readPackageJson() {
    const pkgJsonPath = path.join(process.cwd(), 'package.json');
    const pkgJsonRaw = await fs.readFile(pkgJsonPath, 'utf8');
    const pkgJson: PackageJson = JSON.parse(pkgJsonRaw);
    return pkgJson;
  }
}
