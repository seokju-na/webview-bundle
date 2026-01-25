import { S3Client, type S3ClientConfig } from '@aws-sdk/client-s3';
import { Upload, type Configuration as UploadConfig } from '@aws-sdk/lib-storage';
import type { BaseRemoteUploader, RemoteUploadParams } from '@webview-bundle/config/remote';

export interface AwsS3RemoteUploaderConfig {
  bucket: string;
  key?: string | ((bundleName: string, version: string) => string);
  contentType?: string;
  cacheControl?: string;
  contentDisposition?: string;
  client?: S3Client;
  clientConfig?: S3ClientConfig;
  upload?: UploadConfig;
}

class AwsS3RemoteUploaderImpl implements BaseRemoteUploader {
  _onUploadProgress: ((progress: { loaded?: number; total?: number; part?: number }) => void) | undefined;

  constructor(private readonly config: AwsS3RemoteUploaderConfig) {}

  async upload(params: RemoteUploadParams): Promise<void> {
    const {
      bucket,
      upload: uploadConfig,
      contentType = 'application/webview-bundle',
      cacheControl,
      contentDisposition,
    } = this.config;
    const { bundleName, version, integrity, signature } = params;
    const client = buildS3Client(this.config);
    const metadata: Record<string, string> = {
      'webview-bundle-name': bundleName,
      'webview-bundle-version': version,
    };
    if (integrity != null) {
      metadata['webview-bundle-integrity'] = integrity;
    }
    if (signature != null) {
      metadata['webview-bundle-signature'] = signature;
    }
    const upload = new Upload({
      client,
      params: {
        Bucket: bucket,
        Key: buildKey(this.config, params),
        ContentType: contentType,
        CacheControl: cacheControl,
        ContentDisposition: contentDisposition,
        Metadata: metadata,
      },
      ...uploadConfig,
    });
    upload.on('httpUploadProgress', progress => {
      this._onUploadProgress?.(progress);
    });
    await upload.done();
  }
}

export function awsS3RemoteUploader(config: AwsS3RemoteUploaderConfig): BaseRemoteUploader {
  return new AwsS3RemoteUploaderImpl(config);
}

function buildS3Client(config: AwsS3RemoteUploaderConfig): S3Client {
  if (config?.client != null) {
    return config.client;
  }
  return new S3Client({ ...config.clientConfig });
}

function buildKey(config: AwsS3RemoteUploaderConfig, params: RemoteUploadParams): string {
  if (typeof config.key === 'string') {
    return config.key;
  }
  const { bundleName, version } = params;
  if (typeof config.key === 'function') {
    return config.key(bundleName, version);
  }
  return `bundles/${bundleName}/${bundleName}_${version}.wvb`;
}
