import { WebviewBundleRemote } from '@wvb/remote-aws/cdk';
import { App, CfnOutput, RemovalPolicy, Stack } from 'aws-cdk-lib';

const app = new App();

const bucketName: unknown = app.node.tryGetContext('bucketName');
if (typeof bucketName !== 'string' || bucketName.length === 0) {
  throw new Error('Set a globally unique S3 bucket name with --context bucketName=<name>.');
}

const region: unknown = app.node.tryGetContext('region') ?? 'us-east-1';
if (typeof region !== 'string' || region.length === 0) {
  throw new Error('The region context must be a non-empty AWS region name.');
}

const stack = new Stack(app, 'WvbPlaygroundAwsCdk', {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region,
  },
});
const remote = new WebviewBundleRemote(stack, 'Remote', {
  bucketName,
  bucketProps: { removalPolicy: RemovalPolicy.RETAIN },
  distributionProps: { comment: 'Webview Bundle playground (CDK)' },
});

new CfnOutput(stack, 'BucketName', { value: remote.bucket.bucketName });
new CfnOutput(stack, 'Region', { value: region });
new CfnOutput(stack, 'Endpoint', { value: remote.endpoint });
new CfnOutput(stack, 'CloudfrontDistributionId', { value: remote.distribution.distributionId });
app.synth();
