export interface ExtractConfig {
  /**
   * Webview Bundle file to use for extracting.
   * If not provided, the file at the "outFile" path is used by default.
   */
  file?: string;
  /**
   * Path to extract Webview Bundle files.
   * If not provided, will use Webview Bundle file name as directory.
   */
  outDir?: string;
  /**
   * Don't create extract files on disk, instead just look what inside on the webview bundle file.
   * @default false
   */
  dryRun?: boolean;
  /**
   * Clean up extracted files if out directory already exists.
   * @default false
   */
  clean?: boolean;
}
