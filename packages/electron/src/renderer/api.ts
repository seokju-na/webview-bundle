import type { WebviewBundleApi, WebviewBundleRemoteApi, WebviewBundleUpdaterApi } from '../api-interface.js';
import type { IpcInvoke } from '../ipc-spec.js';

const remoteListBundles: IpcInvoke<'webview-bundle:updater:list-remotes'> = async () => api().remote.listBundles();
const remoteGetInfo: IpcInvoke<'webview-bundle:remote:get-info'> = async bundleName => api().remote.getInfo(bundleName);
const remoteDownload: IpcInvoke<'webview-bundle:remote:download'> = async bundleName =>
  api().remote.download(bundleName);
const remoteDownloadVersion: IpcInvoke<'webview-bundle:remote:download-version'> = async (bundleName, version) =>
  api().remote.downloadVersion(bundleName, version);

export const remote: WebviewBundleRemoteApi = {
  listBundles: remoteListBundles,
  getInfo: remoteGetInfo,
  download: remoteDownload,
  downloadVersion: remoteDownloadVersion,
};

const updaterListRemotes: IpcInvoke<'webview-bundle:updater:list-remotes'> = async () => api().updater.listRemotes();
const updaterGetUpdate: IpcInvoke<'webview-bundle:updater:get-update'> = async bundleName =>
  api().updater.getUpdate(bundleName);
const updateDownloadUpdate: IpcInvoke<'webview-bundle:updater:download-update'> = async (bundleName, version) =>
  api().updater.downloadUpdate(bundleName, version);
const updaterApplyUpdate: IpcInvoke<'webview-bundle:updater:apply-update'> = async (bundleName, version) =>
  api().updater.applyUpdate(bundleName, version);

export const updater: WebviewBundleUpdaterApi = {
  listRemotes: updaterListRemotes,
  getUpdate: updaterGetUpdate,
  downloadUpdate: updateDownloadUpdate,
  applyUpdate: updaterApplyUpdate,
};

function api(): WebviewBundleApi {
  const global = window as any;
  if (global.webviewBundle == null) {
    throw new Error(`Cannot access to webview bundle api.
Make sure to load the preload script before using the api. (via "import { preload } from '@webview-bundle/electron/preload'")`);
  }
  return global.webviewBundle as WebviewBundleApi;
}
