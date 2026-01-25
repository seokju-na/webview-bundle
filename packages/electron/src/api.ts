import type { BundleSourceVersion, BundleUpdateInfo, ListBundleItem, RemoteBundleInfo } from '@wvb/node';

export type { BundleSourceVersion, BundleUpdateInfo, ListBundleItem, RemoteBundleInfo } from '@wvb/node';

export interface WebviewBundleSourceApi {
  listBundles(): Promise<ListBundleItem[]>;
  loadVersion(bundleName: string): Promise<BundleSourceVersion | null>;
  updateVersion(bundleName: string, version: string): Promise<void>;
  filepath(bundleName: string): Promise<string>;
}

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
}

export interface WebviewBundleApi {
  readonly source: WebviewBundleSourceApi;
  readonly remote: WebviewBundleRemoteApi;
  readonly updater: WebviewBundleUpdaterApi;
}
