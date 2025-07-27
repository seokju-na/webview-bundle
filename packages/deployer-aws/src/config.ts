import type { DeployInfo, UploadReleaseInfo } from '@webview-bundle/deployer';
import type {
  AwsClientCommonOptions,
  AwsCloudfrontClientOptions,
  AwsCloudfrontKeyValueStoreClientOptions,
  AwsS3ClientOptions,
} from './aws.js';

export type AwsS3KeyResolver = (info: Pick<UploadReleaseInfo, 'name' | 'version' | 'channel'>) => string;
export type AwsCloudfrontKeyValueStoreKeyResolver = (info: Pick<DeployInfo, 'name' | 'channel'>) => string;

export interface AwsDeployerConfig extends AwsClientCommonOptions {
  s3Bucket: string;
  cloudfrontKeyValueStoreName: string;
  cloudfrontKeyValueStoreKeyResolver?: AwsCloudfrontKeyValueStoreKeyResolver;
  s3KeyResolver?: AwsS3KeyResolver;
  s3ClientOptions?: AwsS3ClientOptions;
  s3ContentType?: string;
  s3CacheControl?: string;
  cloudfrontClientOptions?: AwsCloudfrontClientOptions;
  cloudfrontKeyValueStoreClientOptions?: AwsCloudfrontKeyValueStoreClientOptions;
  versionFile?: string;
}
