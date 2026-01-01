/**
 * Webview Bundle create config.
 */
export interface CreateConfig {
  /**
   * Path to the source directory.
   *
   * All files under this directory will be included in the Webview Bundle.
   * Use `ignore` to exclude files you don't want to pack.
   */
  srcDir?: string;
  /**
   * Out-file path to create webview bundle archive.
   *
   * If not provided, will create file with directory name.
   * If extension is not set, will automatically use `.wvb` extension.
   */
  outFile?: string;
  truncate?: boolean;
  /**
   * Ignore patterns which exclude files from the bundle.
   */
  ignore?: Array<string | RegExp> | ((file: string) => boolean | Promise<boolean>);
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
  headers?:
    | Record<string, HeadersInit>
    | Array<[string, HeadersInit]>
    | ((file: string) => HeadersInit | Promise<HeadersInit>);
}
