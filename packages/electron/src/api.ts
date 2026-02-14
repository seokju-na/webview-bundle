import type {
  BundleSourceVersion,
  BundleUpdateInfo,
  ListBundleItem,
  ListRemoteBundleInfo,
  RemoteBundleInfo,
} from '@wvb/node';
import type { Buffer } from 'node:buffer';

export type {
  BundleSourceVersion,
  BundleUpdateInfo,
  ListBundleItem,
  ListRemoteBundleInfo,
  RemoteBundleInfo,
} from '@wvb/node';

export interface WebviewBundleSourceApi {
  listBundles(): Promise<ListBundleItem[]>;
  loadVersion(bundleName: string): Promise<BundleSourceVersion | null>;
  updateVersion(bundleName: string, version: string): Promise<void>;
  filepath(bundleName: string): Promise<string>;
}

export interface WebviewBundleRemoteApi {
  listBundles(channel?: string): Promise<ListRemoteBundleInfo[]>;
  getInfo(bundleName: string, channel?: string): Promise<RemoteBundleInfo>;
  download(bundleName: string, channel?: string): Promise<[info: RemoteBundleInfo, bundle: Buffer]>;
  downloadVersion(
    bundleName: string,
    version: string
  ): Promise<[info: RemoteBundleInfo, bundle: Buffer]>;
}

export interface WebviewBundleUpdaterApi {
  listRemotes(): Promise<ListRemoteBundleInfo[]>;
  getUpdate(bundleName: string): Promise<BundleUpdateInfo>;
  downloadUpdate(bundleName: string, version?: string): Promise<RemoteBundleInfo>;
}

export interface WebviewBundleApi {
  readonly source: WebviewBundleSourceApi;
  readonly remote: WebviewBundleRemoteApi;
  readonly updater: WebviewBundleUpdaterApi;
}
