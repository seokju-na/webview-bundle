import { type Bundle, type HttpOptions, S3Uploader, type S3UploaderOptions } from '@webview-bundle/node';

export interface RemoteUploadParams {
  bundleName: string;
  version: string;
  bundle: Bundle;
  force: boolean;
}

export interface BaseRemoteUploader {
  upload(params: RemoteUploadParams): Promise<void>;
}

export interface AwsS3RemoteUploaderConfig extends S3UploaderOptions {
  bucket: string;
}

export function awsS3RemoteUploader(config: AwsS3RemoteUploaderConfig): BaseRemoteUploader {
  return {
    upload: async (bundleName, version, bundle) => {
      const { bucket, ...options } = config;
      const uploader = new S3Uploader(bucket, options);
      await uploader.uploadBundle(bundleName, version, bundle);
    },
  };
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
  /**
   * Options for http request.
   */
  http?: HttpOptions;
}
