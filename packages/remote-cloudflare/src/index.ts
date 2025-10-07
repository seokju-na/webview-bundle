import type { RemoteBundleInfo } from '@webview-bundle/node';
import { Hono } from 'hono';
import type { Context } from './context.js';
import { getBundleData } from './operations/getBundleData.js';
import { getBundleStore } from './operations/getBundleStore.js';
import { listAllBundleStores } from './operations/listAllBundleStores.js';

export type { Context } from './context.js';

export type WebviewBundleRemote = Hono<{ Bindings: Context }>;

export function webviewBundleRemote(): WebviewBundleRemote {
  const app = new Hono<{ Bindings: Context }>();

  app.get('/bundles', async c => {
    const stores = await listAllBundleStores(c.env);
    const bundles = stores
      .map(store => {
        if (store.version == null) {
          return null;
        }
        const info: RemoteBundleInfo = {
          name: store.name,
          version: store.version,
        };
        return info;
      })
      .filter(x => x != null);
    return c.json(bundles);
  });

  app.get('/bundles/:name', async c => {
    const bundleName = c.req.param('name');
    const store = await getBundleStore(c.env, bundleName);
    if (store == null || store.version == null) {
      return c.notFound();
    }
    const info: RemoteBundleInfo = {
      name: store.name,
      version: store.version,
    };
    return c.json(info);
  });

  app.get('/bundles/:name/download/:version', async c => {
    const bundleName = c.req.param('name');
    const version = c.req.param('version');
    const data = await getBundleData(c.env, bundleName, version);
    if (data == null) {
      return c.notFound();
    }
    const headers = new Headers();
    if (data.httpMetadata?.contentType != null) {
      headers.append('content-type', data.httpMetadata.contentType);
    }
    if (data.httpMetadata?.contentDisposition != null) {
      headers.append('content-disposition', data.httpMetadata.contentDisposition);
    }
    if (data.httpMetadata?.cacheControl != null) {
      headers.append('cache-control', data.httpMetadata.cacheControl);
    }
    return new Response(data.body, {
      status: 200,
      headers,
    });
  });

  return app;
}

export const wvbRemote = webviewBundleRemote;
