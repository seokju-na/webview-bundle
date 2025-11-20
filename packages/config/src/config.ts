import fs from 'node:fs/promises';
import { createRequire } from 'node:module';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { rolldown } from 'rolldown';
import { DEFAULT_CONFIG_FILES } from './constants.js';
import { fileExists, findNearestPackageJsonFilePath, isEsmFile } from './fs.js';
import { isNodeBuiltin } from './module.js';
import type { RemoteConfig } from './remote/index.js';

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
  remote?: RemoteConfig;
}

interface NodeModuleWithCompile extends NodeJS.Module {
  _compile(code: string, filename: string): any;
}

export async function loadConfigFile(
  filePath?: string,
  cwd: string = process.cwd()
): Promise<{ config: Config; configFile: string; configFileDependencies: string[] } | null> {
  let resolvedFilePath: string | undefined;
  if (filePath != null) {
    resolvedFilePath = path.isAbsolute(filePath) ? filePath : path.resolve(cwd, filePath);
  } else {
    for (const file of DEFAULT_CONFIG_FILES) {
      const candidateFile = path.resolve(cwd, file);
      if (!(await fileExists(candidateFile))) {
        continue;
      }
      resolvedFilePath = candidateFile;
      break;
    }
  }

  if (resolvedFilePath == null) {
    return null;
  }

  const isDeno = typeof process.versions.deno === 'string';
  const isEsm = isDeno ? true : await isEsmFile(resolvedFilePath);
  const bundle = await bundleConfigFile(resolvedFilePath, isEsm, cwd);

  if (isEsm) {
    const pkgJsonFilePath = await findNearestPackageJsonFilePath(cwd);
    let tmpDir: string | null =
      pkgJsonFilePath != null ? path.join(path.dirname(pkgJsonFilePath), '.wvb') : path.join(cwd, '.wvb');
    try {
      await fs.mkdir(tmpDir, { recursive: true });
    } catch (e: any) {
      if (e.code === 'EACCES') {
        tmpDir = null;
      } else {
        throw e;
      }
    }
    const fileName = path.basename(resolvedFilePath);
    const hash = `${Date.now()}-${Math.random().toString(16).slice(2)}`;
    const tmpFilePath =
      tmpDir != null ? path.join(tmpDir, `${fileName}.${hash}.mjs`) : `${resolvedFilePath}.${hash}.mjs`;
    await fs.writeFile(tmpFilePath, bundle.code);
    try {
      const imported = await import(pathToFileURL(tmpFilePath).href);
      const config = imported.default;
      return {
        config,
        configFile: resolvedFilePath,
        configFileDependencies: bundle.dependencies,
      };
    } finally {
      await fs.unlink(tmpFilePath).catch(() => {});
    }
  }

  const _require = createRequire(import.meta.url);
  const ext = path.extname(resolvedFilePath);
  const realFilePath = await fs.realpath(resolvedFilePath);
  const loaderExt = ext in _require.extensions ? ext : '.js';
  const defaultLoader = _require.extensions[loaderExt]!;
  _require.extensions[loaderExt] = (module: NodeJS.Module, filePath: string) => {
    if (filePath === realFilePath) {
      (module as NodeModuleWithCompile)._compile(bundle.code, filePath);
    } else {
      defaultLoader(module, filePath);
    }
  };
  const raw = _require(resolvedFilePath);
  _require.extensions[loaderExt] = defaultLoader;

  const config = raw.__esModule ? raw.default : raw;
  return {
    config,
    configFile: resolvedFilePath,
    configFileDependencies: bundle.dependencies,
  };
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
    external: id => {
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

export interface InlineConfig extends Config {
  configFile?: string | false;
}

export interface ResolvedConfig
  extends Readonly<
    Omit<Config, 'root'> & {
      root: string;
      configFile: string | undefined;
      configFileDependencies: string[] | undefined;
      inlineConfig: InlineConfig;
    }
  > {}

export async function resolveConfig(inlineConfig: InlineConfig): Promise<ResolvedConfig> {
  let config = inlineConfig;
  let configFileDependencies: string[] = [];
  let { configFile } = config;
  if (config.configFile !== false) {
    const loaded = await loadConfigFile(config.configFile, config.root);
    if (loaded != null) {
      config = { ...loaded.config, ...config };
      configFile = loaded.configFile;
      configFileDependencies = loaded.configFileDependencies;
    }
  }
  const root = config.root ?? process.cwd();
  const resolved: ResolvedConfig = {
    root,
    configFile: configFile || undefined,
    configFileDependencies,
    inlineConfig,
  };
  return resolved;
}
