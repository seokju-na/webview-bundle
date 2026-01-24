import { S3Uploader, type S3UploaderOptions } from '@webview-bundle/node';
import type { BaseRemoteUploader } from './remote.js';

export interface AwsS3RemoteUploaderConfig extends Omit<S3UploaderOptions, 'http'> {
  bucket: string;
}

export function awsS3RemoteUploader(config: AwsS3RemoteUploaderConfig): BaseRemoteUploader {
  return {
    upload: async (params, remoteConfig) => {
      const { bundleName, version, bundle } = params;
      const { bucket, ...options } = config;
      const uploader = new S3Uploader(bucket, { ...options, http: remoteConfig.http });
      await uploader.uploadBundle(bundleName, version, bundle);
    },
  };
}
