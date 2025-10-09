import type { Context } from '../types.js';
import { listBundles } from './listBundles.js';

export async function listAllBundles(context: Context): Promise<string[]> {
  let continuationToken: string | undefined;
  const bundles: string[] = [];
  do {
    const result = await listBundles(context, {
      continuationToken,
    });
    bundles.push(...result.bundles);
    continuationToken = result.continuationToken;
  } while (continuationToken != null);
  return bundles;
}
