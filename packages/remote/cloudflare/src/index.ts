import { Hono } from 'hono';
import type { Context } from './context.js';
import { getBundleDataResponse } from './operations/getBundleDataResponse.js';
import { getBundleDeployment } from './operations/getBundleDeployment.js';
import { listAllBundleDeployments } from './operations/listAllBundleDeployments.js';

export type { Context } from './context.js';

export type WebviewBundleRemote = Hono<{ Bindings: Context }>;

export interface WebviewBundleRemoteOptions {
  /** Option to allow downloading other version instead of deployed version */
  allowOtherVersions?: boolean;
}

export function webviewBundleRemote(options: WebviewBundleRemoteOptions = {}): WebviewBundleRemote {
  const { allowOtherVersions = false } = options;

  const app = new Hono<{ Bindings: Context }>();

  app.get('/bundles', async c => {
    const channel = c.req.query('channel');
    const deployments = await listAllBundleDeployments(c.env);
    const bundles = deployments
      .map(x => {
        const version = channel != null ? (x.channels?.[channel] ?? x.version) : x.version;
        if (version == null) {
          return null;
        }
        return { name: x.name, version };
      })
      .filter(x => x != null);
    return c.json(bundles);
  });

  app.get('/bundles/:name', async c => {
    const bundleName = c.req.param('name');
    const channel = c.req.query('channel');
    const deployment = await getBundleDeployment(c.env, bundleName);
    const version = channel != null ? (deployment?.channels?.[channel] ?? deployment?.version) : deployment?.version;
    if (deployment == null || version == null) {
      return c.notFound();
    }
    const resp = await getBundleDataResponse(c.env, bundleName, version);
    if (resp == null) {
      return c.notFound();
    }
    return resp;
  });

  app.get('/bundles/:name/:version', async c => {
    if (!allowOtherVersions) {
      return new Response(null, { status: 403 });
    }
    const bundleName = c.req.param('name');
    const version = c.req.param('version');
    const resp = await getBundleDataResponse(c.env, bundleName, version);
    if (resp == null) {
      return c.notFound();
    }
    return resp;
  });

  return app;
}

export const wvbRemote = webviewBundleRemote;
