import { WvbRemoteProvider } from '@webview-bundle/remote-aws-pulumi';

const remoteProvider = new WvbRemoteProvider('webview-bundle', {
  name: 'wvb4',
  bucketForceDestroy: true,
});

export const bucketName = remoteProvider.bucketName;
export const cloudfrontDistributionDomainName = remoteProvider.cloudfrontDistributionDomainName;
