import type { Context, RemoteBundleDeployment } from '../types.js';
import { listBundleDeployments } from './listBundleDeployments.js';

export async function listAllBundleDeployments(context: Context): Promise<RemoteBundleDeployment[]> {
  let continuationToken: string | undefined;
  const deployments: RemoteBundleDeployment[] = [];
  do {
    const result = await listBundleDeployments(context, {
      continuationToken,
    });
    deployments.push(...result.deployments);
    continuationToken = result.continuationToken;
  } while (continuationToken != null);
  return deployments;
}
