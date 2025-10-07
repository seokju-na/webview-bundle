import type { Context } from '../context.js';
import { type RemoteBundleStore, RemoteBundleStoreSchema } from '../types/store.js';

export interface ListBundleStoresOptions {
  limit?: number;
  cacheTtl?: number;
  cursor?: string;
}

export interface ListBundleStoresResult {
  items: RemoteBundleStore[];
  nextCursor?: string;
}

const MAX_LIMIT = 100;

export async function listBundleStores(
  context: Context,
  options: ListBundleStoresOptions = {}
): Promise<ListBundleStoresResult> {
  const { limit = MAX_LIMIT, cacheTtl, cursor } = options;
  const keysResult = await context.kv.list({
    limit: Math.min(limit, MAX_LIMIT),
    cursor,
  });
  const valuesResult = await context.kv.get(
    keysResult.keys.map(x => x.name),
    {
      type: 'json',
      cacheTtl,
    }
  );
  const items: RemoteBundleStore[] = [];
  for (const value of Object.values(valuesResult)) {
    if (value == null) {
      continue;
    }
    const parsed = RemoteBundleStoreSchema.safeParse(value);
    if (parsed.success) {
      items.push(parsed.data);
    }
  }
  return {
    items,
    nextCursor: keysResult.list_complete ? undefined : keysResult.cursor,
  };
}
