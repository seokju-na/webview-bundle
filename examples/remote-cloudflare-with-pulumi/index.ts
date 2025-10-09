import * as pulumi from '@pulumi/pulumi';
import { WvbRemoteProvider } from '@webview-bundle/remote-cloudflare-pulumi';

const config = new pulumi.Config();

const remoteProvider = new WvbRemoteProvider('webview-bundle', {
  accountId: config.require('accountId'),
  workerSubdomain: {
    enabled: true,
    previewsEnabled: true,
  },
});

export const bucketName = remoteProvider.bucketName;
