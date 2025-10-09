import type { Context } from '../context.js';
import type { RemoteBundleDeployment } from '../types.js';
import { listBundleStores } from './listBundleStores.js';

export interface ListAllBundleDeploymentsOptions {
  cacheTtl?: number;
}

export async function listAllBundleDeployments(
  context: Context,
  options?: ListAllBundleDeploymentsOptions
): Promise<RemoteBundleDeployment[]> {
  const deployments: RemoteBundleDeployment[] = [];
  let cursor: string | undefined;
  do {
    const result = await listBundleStores(context, { cursor, ...options });
    deployments.push(...result.deployments);
    cursor = result.nextCursor;
  } while (cursor != null);
  return deployments;
}
