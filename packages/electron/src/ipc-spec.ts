import type { IpcMainInvokeEvent } from 'electron';
import type { BundleUpdateInfo, RemoteBundleInfo } from './api-interface.js';

export const IpcChannels = {
  Remote: {
    ListBundles: 'webview-bundle:remote:list-bundles',
    GetInfo: 'webview-bundle:remote:get-info',
    Download: 'webview-bundle:remote:download',
    DownloadVersion: 'webview-bundle:remote:download-version',
  },
  Updater: {
    ListRemotes: 'webview-bundle:updater:list-remotes',
    GetUpdate: 'webview-bundle:updater:get-update',
    DownloadUpdate: 'webview-bundle:updater:download-update',
    ApplyUpdate: 'webview-bundle:updater:apply-update',
  },
} as const;

type ValueOf<T> = T[keyof T];
type DeepValueOf<T> = T extends object ? ValueOf<{ [K in keyof T]: DeepValueOf<T[K]> }> : T;

export type IpcChannelScope = Lowercase<keyof typeof IpcChannels>;
export type IpcChannel = DeepValueOf<typeof IpcChannels>;

export type IpcHandler<Return = unknown, Args extends unknown[] = []> = (
  event: IpcMainInvokeEvent,
  ...args: Args
) => Promise<Return>;
export type IpcHandlerSpecs = {
  'webview-bundle:remote:list-bundles': IpcHandler<string[]>;
  'webview-bundle:remote:get-info': IpcHandler<RemoteBundleInfo, [bundleName: string]>;
  'webview-bundle:remote:download': IpcHandler<RemoteBundleInfo, [bundleName: string]>;
  'webview-bundle:remote:download-version': IpcHandler<RemoteBundleInfo, [bundleName: string, version: string]>;
  'webview-bundle:updater:list-remotes': IpcHandler<string[]>;
  'webview-bundle:updater:get-update': IpcHandler<BundleUpdateInfo, [bundleName: string]>;
  'webview-bundle:updater:download-update': IpcHandler<
    RemoteBundleInfo,
    [bundleName: string, version?: string | undefined]
  >;
  'webview-bundle:updater:apply-update': IpcHandler<void, [bundleName: string, version: string]>;
};
export type IpcHandlerSpecsByScope<Scope extends IpcChannelScope> = {
  [K in Extract<keyof IpcHandlerSpecs, `webview-bundle:${Scope}:${string}`>]: IpcHandlerSpecs[K];
};

type IpcHandlerArgs<T extends IpcChannel> = IpcHandlerSpecs[T] extends (
  event: IpcMainInvokeEvent,
  ...args: infer Args
) => any
  ? Args
  : never;
type IpcHandlerReturn<T extends IpcChannel> = IpcHandlerSpecs[T] extends (...args: any[]) => Promise<infer Return>
  ? Return
  : never;

export type IpcInvoke<T extends IpcChannel> = (...args: IpcHandlerArgs<T>) => Promise<IpcHandlerReturn<T>>;
