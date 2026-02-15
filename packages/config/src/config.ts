import type { BuiltinConfig } from './builtin.js';
import type { CreateConfig } from './create.js';
import type { ExtractConfig } from './extract.js';
import type { RemoteConfig } from './remote/index.js';
import type { ServeConfig } from './serve.js';

export type ConfigInputFnObj = () => Config;
export type ConfigInputFnPromise = () => Promise<Config>;
export type ConfigInputFn = () => Config | Promise<Config>;

export type ConfigInput =
  | Config
  | Promise<Config>
  | ConfigInputFnObj
  | ConfigInputFnPromise
  | ConfigInputFn;

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
  /**
   * Path to the source directory.
   *
   * All files under this directory will be included in the Webview Bundle.
   * Use "create.ignore" to exclude files you don't want to pack.
   */
  srcDir?: string;
  /**
   * Directory that out-file should be created.
   * @default ".wvb"
   */
  outDir?: string;
  /**
   * Outfile name to create Webview Bundle archive.
   * If not provided, default to name field in "package.json" with normalized.
   */
  outFile?: string;
  create?: CreateConfig;
  extract?: ExtractConfig;
  remote?: RemoteConfig;
  serve?: ServeConfig;
  builtin?: BuiltinConfig;
}
