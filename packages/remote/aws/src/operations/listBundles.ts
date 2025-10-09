import { ListObjectsV2Command } from '@aws-sdk/client-s3';
import type { Context } from '../types.js';

export interface ListBundlesOptions {
  continuationToken?: string;
}

export interface ListBundlesResult {
  bundles: string[];
  continuationToken?: string;
}

export async function listBundles(context: Context, options?: ListBundlesOptions): Promise<ListBundlesResult> {
  const output = await context.s3Client.send(
    new ListObjectsV2Command({
      Bucket: context.bucketName,
      Prefix: 'bundles/',
      Delimiter: '/',
      ContinuationToken: options?.continuationToken,
    })
  );
  const bundles =
    output.CommonPrefixes?.map(x => {
      const path = x.Prefix || '';
      const bundle = path.replace('bundles/', '').replace('/', '');
      return bundle;
    }).filter(x => x.length > 0) ?? [];
  const result: ListBundlesResult = {
    bundles,
    continuationToken: output.NextContinuationToken,
  };
  return result;
}
