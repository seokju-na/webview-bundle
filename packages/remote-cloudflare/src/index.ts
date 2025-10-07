export type { Context } from './context.js';

export interface RemoteConfig {
  kv: KVNamespace;
  r2: R2Bucket;
}

export function webviewBundleRemote(config: RemoteConfig) {}

export const wvbRemote = webviewBundleRemote;
