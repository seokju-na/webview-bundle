import type { Context } from '../context.js';
import { type RemoteBundleDeployment, RemoteBundleDeploymentSchema } from '../types.js';

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

export async function listBundleStores(
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
  const valuesResult = await context.kv.get(
    keysResult.keys.map(x => x.name),
    {
      type: 'json',
      cacheTtl,
    }
  );
  const deployments: RemoteBundleDeployment[] = [];
  for (const value of Object.values(valuesResult)) {
    if (value == null) {
      continue;
    }
    const parsed = RemoteBundleDeploymentSchema.safeParse(value);
    if (parsed.success) {
      deployments.push(parsed.data);
    }
  }
  return {
    deployments,
    nextCursor: keysResult.list_complete ? undefined : keysResult.cursor,
  };
}
