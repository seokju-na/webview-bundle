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
   * Clean up extracted files if out directory already exists.
   * @default false
   */
  clean?: boolean;
}
