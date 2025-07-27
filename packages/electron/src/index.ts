export type { Cache } from './cache.js';
export { type FSLike, FSLoader, type FSLoaderOptions } from './fs-loader.js';
export {
  type RegisterIpcConfig,
  registerIpc,
  type UnregisterIpc,
} from './ipc.js';
export type { Loader } from './loader.js';
export {
  createProtocolHandler,
  type ProtocolHandler,
  type ProtocolHandlerConfig,
  type RegisterProtocolConfig,
  registerProtocol,
  type UnregisterProtocol,
} from './protocol.js';
export { URI } from './uri.js';

export {
  type WebviewBundleConfig,
  type WebviewBundleOutput,
  webviewBundle,
} from './webview-bundle.js';
