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
    headers.set('content-type', obj.httpMetadata.contentType);
  } else {
    headers.set('content-type', 'application/webview-bundle');
  }
  if (obj.httpMetadata?.contentDisposition != null) {
    headers.set('content-disposition', obj.httpMetadata.contentDisposition);
  }
  if (obj.httpMetadata?.cacheControl != null) {
    headers.set('cache-control', obj.httpMetadata.cacheControl);
  }
  headers.set('webview-bundle-name', bundleName);
  headers.set('webview-bundle-version', version);
  return new Response(obj.body, {
    status: 200,
    headers,
  });
}
