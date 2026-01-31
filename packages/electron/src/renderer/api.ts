import type {
  WebviewBundleApi,
  WebviewBundleRemoteApi,
  WebviewBundleSourceApi,
  WebviewBundleUpdaterApi,
} from '../api.js';
import type { IpcInvoke } from '../ipc-spec.js';

const sourceListBundles: IpcInvoke<'webview-bundle:source:list-bundles'> = async () => api().source.listBundles();
const sourceLoadVersion: IpcInvoke<'webview-bundle:source:load-version'> = async bundleName =>
  api().source.loadVersion(bundleName);
const sourceUpdateVersion: IpcInvoke<'webview-bundle:source:update-version'> = async (bundleName, version) =>
  api().source.updateVersion(bundleName, version);
const sourceFilepath: IpcInvoke<'webview-bundle:source:filepath'> = async bundleName =>
  api().source.filepath(bundleName);

export const source: WebviewBundleSourceApi = {
  listBundles: sourceListBundles,
  loadVersion: sourceLoadVersion,
  updateVersion: sourceUpdateVersion,
  filepath: sourceFilepath,
};

const remoteListBundles: IpcInvoke<'webview-bundle:remote:list-bundles'> = async channel =>
  api().remote.listBundles(channel);
const remoteGetInfo: IpcInvoke<'webview-bundle:remote:get-info'> = async (bundleName, channel) =>
  api().remote.getInfo(bundleName, channel);
const remoteDownload: IpcInvoke<'webview-bundle:remote:download'> = async (bundleName, channel) =>
  api().remote.download(bundleName, channel);
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

export const updater: WebviewBundleUpdaterApi = {
  listRemotes: updaterListRemotes,
  getUpdate: updaterGetUpdate,
  downloadUpdate: updateDownloadUpdate,
};

function api(): WebviewBundleApi {
  const global = window as any;
  if (global.webviewBundle == null) {
    throw new Error(`Cannot access to webview bundle api.
Make sure to load the preload script before using the api. (via "import { preload } from '@wvb/electron/preload'")`);
  }
  return global.webviewBundle as WebviewBundleApi;
}
