import * as pulumi from '@pulumi/pulumi';
import { WvbRemoteProvider } from '@wvb/remote-cloudflare/pulumi';

const config = new pulumi.Config();

export const accountId = config.require('accountId');
export const workerName = 'wvb-playground-remote';

const remote = new WvbRemoteProvider('remote', {
  accountId,
  bucket: {
    name: 'wvb-playground',
  },
  kv: {
    title: 'wvb-playground-remote',
  },
  worker: {
    name: workerName,
    subdomain: { enabled: true, previewsEnabled: false },
  },
});

export const bucketName = remote.bucketName;
export const kvNamespaceId = remote.kvNamespaceId;
export const workerId = remote.workerId;
export const workerVersionId = remote.workerVersionId;
export const workerDeploymentId = remote.workerDeploymentId;
