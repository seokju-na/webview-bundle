import type { Context } from '../context.js';

export async function getBundleData(
  context: Context,
  bundleName: string,
  version: string
): Promise<R2ObjectBody | null> {
  const key = `bundles/${bundleName}/${version}/${bundleName}_${version}.wvb`;
  const obj = await context.r2.get(key);
  if (obj == null) {
    return null;
  }
  return obj;
}
