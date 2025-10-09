import type { Context } from '../context.js';
import { type RemoteBundleDeployment, RemoteBundleDeploymentSchema } from '../types.js';

export async function getBundleDeployment(
  context: Context,
  bundleName: string
): Promise<RemoteBundleDeployment | null> {
  const value = await context.kv.get(bundleName, { type: 'json' });
  const parsed = RemoteBundleDeploymentSchema.safeParse(value);
  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}
