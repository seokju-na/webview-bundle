import type { Bundle, HttpOptions } from '@webview-bundle/node';

export interface RemoteUploadParams {
  bundleName: string;
  version: string;
  bundle: Bundle;
  force: boolean;
}

export interface BaseRemoteUploader {
  upload(params: RemoteUploadParams, config: RemoteConfig): Promise<void>;
}

export interface RemoteDeployParams {
  bundleName: string;
  version: string;
  channel?: string;
}

export interface BaseRemoteDeployer {
  deploy(params: RemoteDeployParams, config: RemoteConfig): Promise<void>;
}

export interface RemoteConfig {
  /**
   * Endpoint to remote server.
   */
  endpoint?: string;
  /**
   * Name of the bundle to be used in remote.
   */
  bundleName?: string;
  uploader?: BaseRemoteUploader;
  deployer?: BaseRemoteDeployer;
  /**
   * Options for http request.
   */
  http?: HttpOptions;
}
