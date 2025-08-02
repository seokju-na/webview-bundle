import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { env } from './env.js';

export type InvokeArgs = Record<string, unknown> | number[] | ArrayBuffer | Uint8Array;

export async function invoke<T>(command: string, args: InvokeArgs = {}): Promise<T> {
  switch (env.platform) {
    case 'electron':
      return window.__WEBVIEW_BUNDLE_ELECTRON_PRELOAD__!.invoke<T>(command, args);
    case 'tauri':
      return tauriInvoke<T>(command, args);
  }
}
