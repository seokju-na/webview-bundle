import type { Context } from '../context.js';
import type { RemoteBundleDeployment } from '../types.js';

export interface ListBundleDeploymentsOptions {
  limit?: number;
  cacheTtl?: number;
  cursor?: string;
}

export interface ListBundleDeploymentsResult {
  deployments: RemoteBundleDeployment[];
  nextCursor?: string;
}

const MAX_LIMIT = 100;

export async function listBundleDeployments(
  context: Context,
  options: ListBundleDeploymentsOptions = {}
): Promise<ListBundleDeploymentsResult> {
  const { limit = MAX_LIMIT, cacheTtl, cursor } = options;
  const keysResult = await context.kv.list({
    limit: Math.min(limit, MAX_LIMIT),
    cursor,
  });
  if (keysResult.keys.length === 0) {
    return { deployments: [], nextCursor: undefined };
  }
  const values = await context.kv.get<RemoteBundleDeployment>(
    keysResult.keys.map(x => x.name),
    {
      type: 'json',
      cacheTtl,
    }
  );
  return {
    deployments: Object.values(values).filter(x => x != null),
    nextCursor: keysResult.list_complete ? undefined : keysResult.cursor,
  };
}
