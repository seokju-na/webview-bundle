export interface ServeConfig {
  /**
   * Webview Bundle file to use for serving with http server.
   * If not provided, the file at the "outFile" path is used by default.
   */
  file?: string;
  /**
   * Specify a port number on which to start the http server.
   * @default 4312
   */
  port?: number;
  /**
   * Disable log output.
   */
  silent?: boolean;
}
