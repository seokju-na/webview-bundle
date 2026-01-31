export type IgnoreConfig = Array<string | RegExp> | ((file: string) => boolean | Promise<boolean>);
export type HeadersConfig =
  | Record<string, HeadersInit>
  | Array<[string, HeadersInit]>
  | ((file: string) => HeadersInit | null | undefined | Promise<HeadersInit | null | undefined>);

/**
 * Webview Bundle create config.
 */
export interface CreateConfig {
  /**
   * Overwrite outfile if file is already exists
   * @default true
   */
  overwrite?: boolean;
  /**
   * Don't create webview bundle file on disk, instead just simulate packing files.
   * @default false
   */
  dryRun?: boolean;
  /**
   * Ignore patterns which exclude files from the bundle.
   */
  ignore?: IgnoreConfig;
  /**
   * Headers to set for each files in the Webview Bundle.
   *
   * @example
   * {
   *   "*.html": {
   *     "cache-control": "max-age=3600",
   *   },
   *   "*.js": ["cache-control", "max-age=0"]
   * }
   */
  headers?: HeadersConfig;
}
