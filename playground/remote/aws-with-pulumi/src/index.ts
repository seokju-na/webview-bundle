import * as aws from '@pulumi/aws';
import * as pulumi from '@pulumi/pulumi';
import { WvbRemoteProvider } from '@wvb/remote-aws/pulumi';

const config = new pulumi.Config();
export const region = aws.config.region ?? 'us-east-1';

const remote = new WvbRemoteProvider('remote', {
  name: `wvb-playground-pulumi-${pulumi.getStack()}`,
  region,
  bucketName: config.require('bucketName'),
  bucketForceDestroy: false,
  lambdaOriginRequest: { timeout: 30, memorySize: 128 },
});

export const bucketName = remote.bucketName;
export const cloudfrontDistributionId = remote.cloudfrontDistributionId;
export const endpoint = pulumi.interpolate`https://${remote.cloudfrontDistributionDomainName}`;
export const lambdaOriginRequestArn = remote.lambdaOriginRequestArn;
