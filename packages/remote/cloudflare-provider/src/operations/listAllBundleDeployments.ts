import type { Context } from '../context.js';
import type { RemoteBundleDeployment } from '../types.js';
import { listBundleDeployments } from './listBundleDeployments.js';

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
    const result = await listBundleDeployments(context, { ...options, cursor });
    deployments.push(...result.deployments);
    cursor = result.nextCursor;
  } while (cursor != null);
  return deployments;
}
