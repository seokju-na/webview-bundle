import path from 'node:path';
import { rolldown } from 'rolldown';
import { fileExists, isEsmFile } from './utils/fs.js';

export type ConfigInputFnObj = () => Config;
export type ConfigInputFnPromise = () => Promise<Config>;
export type ConfigInputFn = () => Config | Promise<Config>;

export type ConfigInput = Config | Promise<Config> | ConfigInputFnObj | ConfigInputFnPromise | ConfigInputFn;

export function defineConfig(config: Config): Config;
export function defineConfig(config: Promise<Config>): Promise<Config>;
export function defineConfig(config: ConfigInputFnObj): ConfigInputFnObj;
export function defineConfig(config: ConfigInputFnPromise): ConfigInputFnPromise;
export function defineConfig(config: ConfigInputFn): ConfigInputFn;
export function defineConfig(config: ConfigInput): ConfigInput;
export function defineConfig(config: ConfigInput): ConfigInput {
  return config;
}

export interface Config {
  /**
   * Project root directory. Can be an absolute path, or a path relative
   * from the current file.
   * @default process.cwd()
   */
  root?: string;
}

const DEFAULT_CONFIG_FILES: string[] = [
  'webview-bundle.config.js',
  'webview-bundle.config.cjs',
  'webview-bundle.config.mjs',
  'webview-bundle.config.ts',
  'webview-bundle.config.cts',
  'webview-bundle.config.mts',
  'wvb.config.js',
  'wvb.config.cjs',
  'wvb.config.mjs',
  'wvb.config.ts',
  'wvb.config.cts',
  'wvb.config.mts',
];

export async function loadConfigFile(
  filepath?: string,
  cwd: string = process.cwd()
): Promise<{ config: Config } | null> {
  const start = performance.now();

  let resolvedPath: string | undefined;
  if (filepath != null) {
    resolvedPath = path.isAbsolute(filepath) ? filepath : path.resolve(cwd, filepath);
  } else {
    for (const file of DEFAULT_CONFIG_FILES) {
      const candidateFile = path.resolve(cwd, file);
      if (!(await fileExists(candidateFile))) {
        continue;
      }
      resolvedPath = candidateFile;
      break;
    }
  }

  if (resolvedPath == null) {
    return null;
  }

  const isEsm = await isEsmFile(resolvedPath);

  throw new Error('TODO');
}

interface BundledFile {
  fileName: string;
  content: string | Uint8Array;
  isMain: boolean;
}

async function bundleConfigFile(filepath: string, isEsm: boolean): Promise<BundledFile[]> {
  const bundle = await rolldown({
    input: filepath,
    platform: 'node',
    external: () => false,
    transform: {
      target: `node${process.versions.node}`,
    },
  });
  const ext = isEsm ? 'mjs' : 'cjs';
  const { output } = await bundle.generate({
    format: isEsm ? 'esm' : 'cjs',
    sourcemap: 'inline',
    minify: false,
    hashCharacters: 'hex',
    entryFileNames: `[name].${ext}`,
    chunkFileNames: `[name]-[hash].${ext}`,
  });
  const codes: BundledFile[] = [];
  for (const out of output) {
    const code: BundledFile = {
      fileName: out.fileName,
      content: out.type === 'chunk' ? out.code : out.source,
      isMain: out.type === 'chunk' && out.isEntry,
    };
    codes.push(code);
  }
  return codes;
}

export interface ResolvedConfig
  extends Readonly<
    Omit<Config, 'root'> & {
      root: string;
    }
  > {}

export async function resolveConfig(): Promise<ResolvedConfig> {
  throw new Error('TODO');
}
