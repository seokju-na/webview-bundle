import { contextBridge, ipcRenderer } from 'electron';
import type { WebviewBundleApi } from '../api-interface.js';

import { IpcChannels } from '../ipc-spec.js';

export function preload(): void {
  const api: WebviewBundleApi = {
    remote: {
      listBundles: (...args: unknown[]) => ipcRenderer.invoke(IpcChannels.Remote.ListBundles, ...args),
      getInfo: (...args: unknown[]) => ipcRenderer.invoke(IpcChannels.Remote.GetInfo, ...args),
      download: (...args: unknown[]) => ipcRenderer.invoke(IpcChannels.Remote.Download, ...args),
      downloadVersion: (...args: unknown[]) => ipcRenderer.invoke(IpcChannels.Remote.DownloadVersion, ...args),
    },
    updater: {
      listRemotes: (...args: unknown[]) => ipcRenderer.invoke(IpcChannels.Updater.ListRemotes, ...args),
      getUpdate: (...args: unknown[]) => ipcRenderer.invoke(IpcChannels.Updater.GetUpdate, ...args),
      downloadUpdate: (...args: unknown[]) => ipcRenderer.invoke(IpcChannels.Updater.DownloadUpdate, ...args),
      applyUpdate: (...args: unknown[]) => ipcRenderer.invoke(IpcChannels.Updater.ApplyUpdate, ...args),
    },
  };
  contextBridge.exposeInMainWorld('webviewBundle', api);
}
