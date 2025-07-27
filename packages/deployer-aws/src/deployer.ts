import { DescribeKeyValueStoreCommand } from '@aws-sdk/client-cloudfront';
import {
  DescribeKeyValueStoreCommand as DescribeKeyCommand,
  PutKeyCommand,
} from '@aws-sdk/client-cloudfront-keyvaluestore';
import { HeadObjectCommand, PutObjectCommand } from '@aws-sdk/client-s3';
import {
  Deployer,
  type DeployInfo,
  ReleaseAlreadyUploadedError,
  UnexpectedError,
  type UploadReleaseInfo,
} from '@webview-bundle/deployer';
import * as aws from './aws.js';
import type { AwsCloudfrontKeyValueStoreKeyResolver, AwsDeployerConfig, AwsS3KeyResolver } from './config.js';

export class AwsDeployer extends Deployer {
  readonly name = 'aws';

  constructor(public readonly config: AwsDeployerConfig) {
    super();
  }

  async uploadRelease(info: UploadReleaseInfo): Promise<void> {
    const { name, force = false, bundle, version } = info;
    const {
      s3Bucket,
      s3KeyResolver = defaultS3KeyResolver,
      s3ContentType = 'application/webview-bundle',
      s3CacheControl = 's-maxage=31536000, max-age=0',
      versionFile,
    } = this.config;
    const s3 = this.getS3Client();
    const key = s3KeyResolver(info);
    if (!force) {
      const exists = await this.isS3ObjectExists(s3, s3Bucket, key);
      if (exists) {
        throw new ReleaseAlreadyUploadedError(info);
      }
    }
    const bundleData = await this.writeBundleVersion(bundle, versionFile, version);
    try {
      await s3.send(
        new PutObjectCommand({
          Bucket: s3Bucket,
          Key: key,
          Body: bundleData,
          ContentType: s3ContentType,
          CacheControl: s3CacheControl,
          Metadata: {
            'x-webview-bundle-name': name,
            'x-webview-bundle-version': version,
          },
        })
      );
    } catch (e) {
      throw new UnexpectedError('fail to upload release into s3 bucket', e);
    }
  }

  async deploy(info: DeployInfo): Promise<void> {
    const { version } = info;
    const {
      cloudfrontKeyValueStoreName,
      cloudfrontKeyValueStoreKeyResolver = defaultCloudfrontKeyValueStoreKeyResolver,
    } = this.config;
    const cloudfront = this.getCloudfrontClient();
    let storeArn: string;
    try {
      const resp = await cloudfront.send(
        new DescribeKeyValueStoreCommand({
          Name: cloudfrontKeyValueStoreName,
        })
      );
      storeArn = resp.KeyValueStore!.ARN!;
    } catch (e) {
      throw new UnexpectedError(`fail to get cloudfront key value store (name: ${cloudfrontKeyValueStoreName})`, e);
    }
    const cloudfrontKv = this.getCloudfrontKeyValueStoreClient();
    let etag: string;
    try {
      const resp = await cloudfrontKv.send(
        new DescribeKeyCommand({
          KvsARN: storeArn,
        })
      );
      etag = resp.ETag!;
    } catch (e) {
      throw new UnexpectedError(`fail to get etag from cloudfront key value store (arn: ${storeArn})`, e);
    }
    const key = cloudfrontKeyValueStoreKeyResolver(info);
    try {
      await cloudfrontKv.send(
        new PutKeyCommand({
          KvsARN: storeArn,
          Key: key,
          Value: version,
          IfMatch: etag,
        })
      );
    } catch (e) {
      throw new UnexpectedError(
        `fail to update version into cloudfront key value store (arn: ${storeArn}, etag: ${etag})`,
        e
      );
    }
    return Promise.resolve(undefined);
  }

  private getS3Client() {
    const { credential, region, s3ClientOptions } = this.config;
    const client = aws.s3Client({
      credential,
      region,
      ...s3ClientOptions,
    });
    return client;
  }

  private getCloudfrontClient() {
    const { credential, region, cloudfrontClientOptions } = this.config;
    const client = aws.cloudfrontClient({
      credential,
      region,
      ...cloudfrontClientOptions,
    });
    return client;
  }

  private getCloudfrontKeyValueStoreClient() {
    const { credential, region, cloudfrontKeyValueStoreClientOptions } = this.config;
    const client = aws.cloudfrontKeyValueStoreClient({
      credential,
      region,
      ...cloudfrontKeyValueStoreClientOptions,
    });
    return client;
  }

  private async isS3ObjectExists(client: ReturnType<typeof aws.s3Client>, bucket: string, key: string) {
    try {
      await client.send(
        new HeadObjectCommand({
          Bucket: bucket,
          Key: key,
        })
      );
      return true;
    } catch {
      return false;
    }
  }
}

export const defaultS3KeyResolver: AwsS3KeyResolver = info => {
  const { name, version, channel } = info;
  if (channel != null) {
    return `bundles/${name}/${version}/${channel}/bundle.wvb`;
  }
  return `bundles/${name}/${version}/bundle.wvb`;
};

export const defaultCloudfrontKeyValueStoreKeyResolver: AwsCloudfrontKeyValueStoreKeyResolver = info => {
  const { name, channel } = info;
  if (channel != null) {
    return `bundles/${name}/${channel}`;
  }
  return `bundles/${name}`;
};
