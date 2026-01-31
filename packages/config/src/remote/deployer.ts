export interface RemoteDeployParams {
  bundleName: string;
  version: string;
  channel?: string;
}

export interface BaseRemoteDeployer {
  deploy(params: RemoteDeployParams): Promise<void>;
}
