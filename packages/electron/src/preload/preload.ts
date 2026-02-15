import { contextBridge, ipcRenderer } from 'electron';
import type { WebviewBundleApi } from '../api.js';
import { IpcChannels } from '../ipc-spec.js';

export function preload(): void {
  contextBridge.exposeInMainWorld('webviewBundle', buildApi());
}

function buildApi(): WebviewBundleApi {
  const api = {};
  traverseIpcChannels(IpcChannels, [], (paths, channel) => {
    setApi(api, paths, channel);
  });
  return api as WebviewBundleApi;
}

function setApi(api: any, paths: string[], channel: string): void {
  const apiName = paths[paths.length - 1];
  if (apiName == null) {
    return;
  }
  const target = paths.slice(0, -1).reduce((acc, key) => {
    if (acc[key] == null || typeof acc[key] !== 'object') {
      acc[key] = {};
    }
    return acc[key];
  }, api);
  target[apiName] = (...args: unknown[]) => ipcRenderer.invoke(channel, ...args);
}

function traverseIpcChannels(
  channels: object,
  paths: string[],
  callback: (paths: string[], channel: string) => void
): void {
  for (const [key, value] of Object.entries(channels)) {
    const k = toApiKey(key);
    if (typeof value === 'string') {
      callback([...paths, k], value);
    } else if (typeof value === 'object') {
      traverseIpcChannels(value, [...paths, k], callback);
    }
  }
}

function toApiKey(val: string): string {
  // ListBundles -> listBundles
  const [first, ...rest] = val;
  return [first?.toLowerCase(), ...rest].filter(x => x != null).join('');
}
