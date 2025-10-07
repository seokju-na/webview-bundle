import type { Context } from '../context.js';
import type { RemoteBundleStore } from '../types/store.js';
import { listBundleStores } from './listBundleStores.js';

export interface ListAllBundleStoresOptions {
  cacheTtl?: number;
}

export async function listAllBundleStores(
  context: Context,
  options?: ListAllBundleStoresOptions
): Promise<RemoteBundleStore[]> {
  const items: RemoteBundleStore[] = [];
  let cursor: string | undefined;
  do {
    const result = await listBundleStores(context, { cursor, ...options });
    items.push(...result.items);
    cursor = result.nextCursor;
  } while (cursor != null);
  return items;
}
