import type { BundleUpdateInfo, RemoteBundleInfo } from '@webview-bundle/node';

export type { BundleUpdateInfo, RemoteBundleInfo } from '@webview-bundle/node';

export interface WebviewBundleRemoteApi {
  listBundles(): Promise<string[]>;
  getInfo(bundleName: string): Promise<RemoteBundleInfo>;
  download(bundleName: string): Promise<RemoteBundleInfo>;
  downloadVersion(bundleName: string, version: string): Promise<RemoteBundleInfo>;
}

export interface WebviewBundleUpdaterApi {
  listRemotes(): Promise<string[]>;
  getUpdate(bundleName: string): Promise<BundleUpdateInfo>;
  downloadUpdate(bundleName: string, version?: string): Promise<RemoteBundleInfo>;
  applyUpdate(bundleName: string, version: string): Promise<void>;
}

export interface WebviewBundleApi {
  readonly remote: WebviewBundleRemoteApi;
  readonly updater: WebviewBundleUpdaterApi;
}
