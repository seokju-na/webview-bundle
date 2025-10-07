import type { Context } from '../context.js';
import { type RemoteBundleStore, RemoteBundleStoreSchema } from '../types/store.js';

export async function getBundleStore(context: Context, bundleName: string): Promise<RemoteBundleStore | null> {
  const value = await context.kv.get(bundleName, { type: 'json' });
  const parsed = RemoteBundleStoreSchema.safeParse(value);
  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}
