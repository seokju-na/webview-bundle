import type { Remote, Updater } from '@webview-bundle/node';
import { ipcMain } from 'electron';
import { IpcChannels, type IpcHandlerSpecsByScope } from './ipc-spec.js';
import type { WebviewBundle } from './webview-bundle.js';

export function registerIpc(wvb: WebviewBundle): void {
  registerRemoteIpc(wvb);
  registerUpdaterIpc(wvb);
}

function registerRemoteIpc(wvb: WebviewBundle): void {
  function remote(): Remote {
    if (wvb.remote == null) {
      throw new Error('remote is not initialized.');
    }
    return wvb.remote;
  }
  const handlers = {
    [IpcChannels.Remote.ListBundles]: async () => remote().listBundles(),
    [IpcChannels.Remote.GetInfo]: async (_, bundleName) => remote().getInfo(bundleName),
    [IpcChannels.Remote.Download]: async (_, bundleName) => {
      const [info] = await remote().download(bundleName);
      return info;
    },
    [IpcChannels.Remote.DownloadVersion]: async (_, bundleName, version) => {
      const [info] = await remote().downloadVersion(bundleName, version);
      return info;
    },
  } satisfies IpcHandlerSpecsByScope<'remote'>;

  for (const [channel, handler] of Object.entries(handlers)) {
    ipcMain.handle(channel, handler);
  }
}

function registerUpdaterIpc(wvb: WebviewBundle): void {
  function updater(): Updater {
    if (wvb.updater == null) {
      throw new Error('updater is not initialized.');
    }
    return wvb.updater;
  }
  const handlers = {
    [IpcChannels.Updater.ListRemotes]: async () => updater().listRemotes(),
    [IpcChannels.Updater.GetUpdate]: async (_, remoteName) => updater().getUpdate(remoteName),
    [IpcChannels.Updater.DownloadUpdate]: async (_, remoteName, version) => {
      const info = await updater().downloadUpdate(remoteName, version);
      return info;
    },
    [IpcChannels.Updater.ApplyUpdate]: async (_, remoteName, version) => {
      await updater().applyUpdate(remoteName, version);
    },
  } satisfies IpcHandlerSpecsByScope<'updater'>;

  for (const [channel, handler] of Object.entries(handlers)) {
    ipcMain.handle(channel, handler);
  }
}
