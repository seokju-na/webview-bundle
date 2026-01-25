import type { RemoteConfig } from './remote.js';

export interface RemoteDeployParams {
  bundleName: string;
  version: string;
  channel?: string;
}

export interface BaseRemoteDeployer {
  deploy(params: RemoteDeployParams, config: RemoteConfig): Promise<void>;
}
