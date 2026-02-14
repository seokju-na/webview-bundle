import { ListObjectsV2Command } from '@aws-sdk/client-s3';
import type { Context, RemoteBundleDeployment } from '../types.js';
import { getBundleDeployment } from './getBundleDeployment.js';

export interface ListBundleDeploymentsOptions {
  continuationToken?: string;
}

export interface ListBundleDeploymentsResult {
  deployments: RemoteBundleDeployment[];
  continuationToken?: string;
}

export async function listBundleDeployments(
  context: Context,
  options?: ListBundleDeploymentsOptions
): Promise<ListBundleDeploymentsResult> {
  const output = await context.s3Client.send(
    new ListObjectsV2Command({
      Bucket: context.bucketName,
      Prefix: 'bundles/',
      Delimiter: '/',
      ContinuationToken: options?.continuationToken,
    })
  );
  const bundleNames =
    output.CommonPrefixes?.map(x => {
      const path = x.Prefix != null && x.Prefix.length > 0 ? x.Prefix : '';
      const bundle = path.replace('bundles/', '').replace('/', '');
      return bundle;
    }).filter(x => x.length > 0) ?? [];
  const deployments = await Promise.all(
    bundleNames.map(async bundleName => {
      try {
        const deployment = await getBundleDeployment(context, bundleName);
        return deployment;
      } catch {
        return null;
      }
    })
  );
  const result: ListBundleDeploymentsResult = {
    deployments: deployments.filter(x => x != null),
    continuationToken: output.NextContinuationToken,
  };
  return result;
}
