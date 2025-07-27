import { contextBridge, ipcRenderer } from 'electron/renderer';
import { IpcChannels } from '../ipc-channels.js';

export function preload(): void {
  contextBridge.exposeInMainWorld('__WEBVIEW_BUNDLE__', {
    getBundleVersion: (name: string) => ipcRenderer.invoke(IpcChannels.GetBundleVersion, { name }),
  });
}
