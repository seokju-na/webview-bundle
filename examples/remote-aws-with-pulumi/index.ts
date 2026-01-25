import { WvbRemoteProvider } from '@wvb/remote-aws-provider-pulumi';

const remoteProvider = new WvbRemoteProvider('webview-bundle', {
  bucketForceDestroy: true,
});

export const bucketName = remoteProvider.bucketName;
export const cloudfrontDistributionDomainName = remoteProvider.cloudfrontDistributionDomainName;
