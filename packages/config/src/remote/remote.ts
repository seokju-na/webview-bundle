import type { Bundle, HttpOptions } from '@webview-bundle/node';

export interface RemoteUploadParams {
  bundleName: string;
  version: string;
  bundle: Bundle;
  force: boolean;
  integrity?: string;
  signature?: string;
}

export interface BaseRemoteUploader {
  _onUploadProgress?: (progress: { loaded?: number; total?: number; part?: number }) => void;
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
