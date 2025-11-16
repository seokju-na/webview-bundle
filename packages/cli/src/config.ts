import { createRequire } from 'node:module';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { rolldown } from 'rolldown';
import { fileExists, isEsmFile } from './fs.js';
import { isNodeBuiltin } from './module.js';

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
  'wvb.config.js',
  'wvb.config.cjs',
  'wvb.config.mjs',
  'wvb.config.ts',
  'wvb.config.cts',
  'wvb.config.mts',
  'webview-bundle.config.js',
  'webview-bundle.config.cjs',
  'webview-bundle.config.mjs',
  'webview-bundle.config.ts',
  'webview-bundle.config.cts',
  'webview-bundle.config.mts',
];

export async function loadConfigFile(
  filepath?: string,
  cwd: string = process.cwd()
): Promise<{ config: Config } | null> {
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
  const files = await bundleConfigFile(resolvedPath, isEsm, cwd);
  console.log(files);

  throw new Error('TODO');
}

async function bundleConfigFile(
  absoluteFilepath: string,
  isEsm: boolean,
  cwd?: string
): Promise<{
  code: string;
  dependencies: string[];
}> {
  const dirname = path.dirname(absoluteFilepath);
  const filename = absoluteFilepath;
  const bundle = await rolldown({
    input: absoluteFilepath,
    platform: 'node',
    external: (id, importer) => {
      console.log(id, importer);
      if (!id || id.startsWith('\0') || id.startsWith('.') || id.startsWith('#') || path.isAbsolute(id)) {
        return false;
      }
      if (isNodeBuiltin(id) || id.startsWith('npm:')) {
        return true;
      }
      return true;
    },
    transform: {
      target: `node${process.versions.node}`,
      define: {
        __dirname: JSON.stringify(dirname),
        __filename: JSON.stringify(filename),
        'import.meta.url': JSON.stringify(pathToFileURL(filename).href),
        'import.meta.filename': JSON.stringify(filename),
        'import.meta.dirname': JSON.stringify(dirname),
        'import.meta.env': 'process.env',
      },
    },
    cwd,
  });

  const bundleOutput = await bundle.generate({
    format: isEsm ? 'esm' : 'cjs',
    sourcemap: 'inline',
    keepNames: true,
    inlineDynamicImports: true,
  });
  if (bundleOutput.output[0] == null || bundleOutput.output[0].type !== 'chunk') {
    throw new Error('no output chunk found');
  }
  const dependencies = await bundle.watchFiles;
  return {
    code: bundleOutput.output[0].code,
    dependencies,
  };
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
