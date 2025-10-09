import { GetObjectCommand } from '@aws-sdk/client-s3';
import { type Context, type RemoteBundleDeployment, RemoteBundleDeploymentSchema } from '../types.js';
import { isNoSuchKeyError } from '../utils.js';

export async function getBundleDeployment(
  context: Context,
  bundleName: string
): Promise<RemoteBundleDeployment | null> {
  try {
    const output = await context.s3Client.send(
      new GetObjectCommand({
        Bucket: context.bucketName,
        Key: `bundles/${bundleName}/deployment.json`,
      })
    );
    const raw = await output.Body?.transformToString('utf-8');
    if (raw == null) {
      throw new Error('Response body is empty');
    }
    const json = JSON.parse(raw);
    return RemoteBundleDeploymentSchema.parse(json);
  } catch (e) {
    if (isNoSuchKeyError(e)) {
      return null;
    }
    throw e;
  }
}
