import { CloudFrontClient, type CloudFrontClientConfig } from '@aws-sdk/client-cloudfront';
import {
  CloudFrontKeyValueStoreClient,
  type CloudFrontKeyValueStoreClientConfig,
} from '@aws-sdk/client-cloudfront-keyvaluestore';
import { S3Client, type S3ClientConfig } from '@aws-sdk/client-s3';
import type { AwsCredentialIdentity, AwsCredentialIdentityProvider } from '@smithy/types';

export interface AwsClientCommonOptions {
  credential?: AwsCredentialIdentity | AwsCredentialIdentityProvider;
  region?: string;
}

export type AwsS3ClientOptions = Omit<S3ClientConfig, keyof AwsClientCommonOptions>;
export type AwsS3Client = S3Client;

export type AwsCloudfrontClientOptions = Omit<CloudFrontClientConfig, keyof AwsClientCommonOptions>;
export type AwsCloudfrontClient = CloudFrontClient;

export type AwsCloudfrontKeyValueStoreClientOptions = Omit<
  CloudFrontKeyValueStoreClientConfig,
  keyof AwsClientCommonOptions
>;
export type AwsCloudfrontKeyValueStoreClient = CloudFrontKeyValueStoreClient;

export function s3Client(options: AwsS3ClientOptions & AwsClientCommonOptions = {}): AwsS3Client {
  return new S3Client(options);
}

export function cloudfrontClient(
  options: AwsCloudfrontClientOptions & AwsClientCommonOptions = {}
): AwsCloudfrontClient {
  return new CloudFrontClient(options);
}

export function cloudfrontKeyValueStoreClient(
  options: AwsCloudfrontKeyValueStoreClientOptions & AwsClientCommonOptions = {}
): AwsCloudfrontKeyValueStoreClient {
  return new CloudFrontKeyValueStoreClient(options);
}
