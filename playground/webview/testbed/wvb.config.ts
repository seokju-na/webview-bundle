import { defineConfig } from '@wvb/cli';
import { awsRemote } from '@wvb/remote-aws';
import { cloudflareRemote } from '@wvb/remote-cloudflare';
import { loadEnv } from '@wvb-playground/env';
import { getPickedVersion, versionDistDir } from '@wvb-playground/versions';

const env = loadEnv();
const version = getPickedVersion();

const { REMOTE_PROVIDER: remoteProvider = 'cloudflare' } = process.env;
let remote: any;
switch (remoteProvider) {
  case 'aws':
    remote = awsRemote({
      bucket: env.aws.bucket,
    });
    break;
  case 'cloudflare':
    remote = cloudflareRemote({
      bucket: env.cloudflare.bucket,
      accountId: env.cloudflare.accountId,
      kvNamespaceId: env.cloudflare.kvNamespaceId,
      cloudflareConfig: {
        apiToken: env.cloudflare.apiToken,
      },
      aws: {
        region: 'auto',
        credentials: {
          accessKeyId: env.cloudflare.accessKeyId,
          secretAccessKey: env.cloudflare.secretAccessKey,
        },
      },
    });
    break;
  default:
    throw new Error(`Unknown remote provider: ${remoteProvider}`);
}

export default defineConfig({
  pack: {
    srcDir: versionDistDir(version, import.meta.dirname),
    outFile: `./.wvb/testbed_${version}.wvb`,
  },
  remote: {
    ...remote,
    version,
    packBeforeUpload: true,
    bundleName: 'testbed',
  },
});
