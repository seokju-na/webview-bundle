import type { Context } from '../context.js';

export async function getBundleDataResponse(
  context: Context,
  bundleName: string,
  version: string
): Promise<Response | null> {
  const key = `bundles/${bundleName}/${version}/${bundleName}_${version}.wvb`;
  const obj = await context.r2.get(key);
  if (obj == null) {
    return null;
  }
  const headers = new Headers();
  if (obj.httpMetadata?.contentType != null) {
    headers.append('content-type', obj.httpMetadata.contentType);
  }
  if (obj.httpMetadata?.contentDisposition != null) {
    headers.append('content-disposition', obj.httpMetadata.contentDisposition);
  }
  if (obj.httpMetadata?.cacheControl != null) {
    headers.append('cache-control', obj.httpMetadata.cacheControl);
  }
  return new Response(obj.body, {
    status: 200,
    headers,
  });
}
