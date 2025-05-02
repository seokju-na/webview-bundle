import path from 'node:path';
import type { Writable } from 'node:stream';
import { build } from 'esbuild';
import glob from 'fast-glob';

interface Config {
  cwd?: string;
  sourceFilePattern?: string;
  outdir: {
    cjs: string;
    esm: string;
  };
  stdout?: Writable;
  stderr?: Writable;
  platform?: 'browser' | 'node' | 'neutral';
  target?: string | string[];
  external?: string[];
}

export async function buildJSFiles(
  src: string,
  {
    cwd = process.cwd(),
    sourceFilePattern = '**/*.{ts,tsx,mts,cts,js,jsx,mjs,cjs}',
    outdir,
    stdout = process.stdout,
    stderr = process.stderr,
    ...options
  }: Config
) {
  const absoluteFiles = await glob(sourceFilePattern, {
    cwd: path.join(cwd, src),
    onlyFiles: true,
    followSymbolicLinks: false,
    dot: true,
    absolute: true,
  });
  const files = absoluteFiles.map(x => path.relative(process.cwd(), x));
  stdout.write(`Build JS files (cjs: ${outdir.cjs}, esm: ${outdir.esm}):\n`);
  for (const file of files) {
    const relativePath = path.relative(cwd, file);
    stdout.write(`  ${relativePath}\n`);
  }
  await Promise.all([buildEsm(files, outdir.esm, options), buildCjs(files, outdir.cjs, options)]);
}

type BuildOptions = Pick<Config, 'platform' | 'target' | 'external'>;

async function buildEsm(entryPoints: string[], outdir: string, { platform, target, external = [] }: BuildOptions = {}) {
  await build({
    entryPoints,
    outdir,
    bundle: true,
    format: 'esm',
    platform,
    target,
    external,
  });
}

async function buildCjs(entryPoints: string[], outdir: string, { platform, target, external = [] }: BuildOptions = {}) {
  await build({
    entryPoints,
    outdir,
    bundle: true,
    format: 'cjs',
    platform,
    target,
    external,
  });
}
