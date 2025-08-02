import type { InvokeArgs } from './invoke.js';

declare global {
  interface Window {
    __WEBVIEW_BUNDLE_ELECTRON_PRELOAD__?: {
      invoke: <T>(command: string, args?: InvokeArgs) => Promise<T>;
    };
  }
}
