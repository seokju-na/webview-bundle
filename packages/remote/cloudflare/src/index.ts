import type { BaseRemoteDeployer, BaseRemoteUploader } from '@wvb/config/remote';
import type { AwsRemoteConfig } from '@wvb/remote-aws';
import { type CloudflareRemoteDeployerConfig, cloudflareRemoteDeployer } from './deployer.js';
import { type CloudflareRemoteUploaderConfig, cloudflareRemoteUploader } from './uploader.js';
import type { CloudflareClientConfigLike, PartialBy } from './utils.js';

export interface CloudflareRemoteConfig extends CloudflareClientConfigLike {
  bucket: string;
  accountId: string;
  namespaceId: string;
  uploader?: PartialBy<CloudflareRemoteUploaderConfig, 'accountId' | 'bucket'>;
  deployer?: PartialBy<CloudflareRemoteDeployerConfig, 'accountId' | 'namespaceId'>;
  s3?: AwsRemoteConfig;
}

export interface CloudflareRemote {
  uploader: BaseRemoteUploader;
  deployer: BaseRemoteDeployer;
}

export function cloudflareRemote(config: CloudflareRemoteConfig): CloudflareRemote {
  const uploader = cloudflareRemoteUploader({
    bucket: config.bucket,
    accountId: config.accountId,
    ...config.uploader,
    s3ClientConfig: {
      ...config.s3,
      ...config.uploader?.s3ClientConfig,
    },
  });
  const deployer = cloudflareRemoteDeployer({
    accountId: config.accountId,
    namespaceId: config.namespaceId,
    ...config.deployer,
    cloudflare: config.deployer?.cloudflare ?? config.cloudflare,
    cloudflareConfig: {
      ...config.cloudflareConfig,
      ...config.deployer?.cloudflareConfig,
    },
  });
  const remote: CloudflareRemote = {
    uploader,
    deployer,
  };
  return remote;
}
