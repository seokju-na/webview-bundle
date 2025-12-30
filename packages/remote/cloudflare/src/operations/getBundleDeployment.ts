import type { Context } from '../context.js';
import type { RemoteBundleDeployment } from '../types.js';

export async function getBundleDeployment(
  context: Context,
  bundleName: string
): Promise<RemoteBundleDeployment | null> {
  return context.kv.get<RemoteBundleDeployment>(bundleName, { type: 'json' });
}
