import path from 'node:path';
import { readBundle, S3Uploader } from '@webview-bundle/node';
import { Command, Option } from 'clipanion';

export class RemoteS3UploadCommand extends Command {
  static paths = [['remote', 's3', 'upload']];

  readonly file = Option.String({
    name: 'FILE',
    required: true,
  });
  readonly version = Option.String({
    name: 'VERSION',
    required: true,
  });
  readonly name = Option.String('--name,-N', {
    description: 'Bundle name to upload. Default to file name.',
  });
  readonly bucket = Option.String('--bucket,-B', {
    required: true,
    description: 'S3 bucket name to upload bundle.',
  });
  readonly accessKeyId = Option.String('--access-key-id', {
    description: 'AWS access key id.',
    env: 'AWS_ACCESS_KEY_ID',
  });
  readonly secretAccessKey = Option.String('--secret-access-key', {
    description: 'AWS secret access key.',
    env: 'AWS_SECRET_ACCESS_KEY',
  });
  readonly endpoint = Option.String('--endpoint,-E', {
    description: 'Endpoint for S3.',
    env: 'AWS_ENDPOINT_URL',
  });
  readonly region = Option.String('--region,-R', {
    description: 'Region for S3.',
    env: 'AWS_REGION',
  });

  async execute() {
    const filepath = path.isAbsolute(this.file) ? this.file : path.join(process.cwd(), this.file);
    const bundle = await readBundle(filepath);
    const uploader = new S3Uploader(this.bucket, {
      accessKeyId: this.accessKeyId,
      secretAccessKey: this.secretAccessKey,
      endpoint: this.endpoint,
      region: this.region,
    });
    let bundleName = this.name ?? path.basename(filepath);
    if (bundleName.endsWith('.wvb')) {
      bundleName = bundleName.replace(/\.wvb$/, '');
    }
    await uploader.uploadBundle(bundleName, this.version, bundle);
  }
}
