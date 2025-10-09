import { Hono } from 'hono';
import type { HTTPResponseError } from 'hono/types';
import { ZodError } from 'zod/v4';
import type { Context } from './context.js';
import { getBundleDataResponse } from './operations/getBundleDataResponse.js';
import { getBundleDeployment } from './operations/getBundleDeployment.js';
import { listAllBundleDeployments } from './operations/listAllBundleDeployments.js';
import type { RemoteBundleInfo } from './types.js';

export type { Context } from './context.js';

export type WebviewBundleRemote = Hono<{ Bindings: Context }>;

export interface WebviewBundleRemoteOptions {
  allowOnlyLatest?: boolean;
}

export function webviewBundleRemote(options: WebviewBundleRemoteOptions = {}): WebviewBundleRemote {
  const { allowOnlyLatest = false } = options;

  const app = new Hono<{ Bindings: Context }>();

  app.get('/bundles', async c => {
    const deployments = await listAllBundleDeployments(c.env);
    const bundles = deployments
      .map(deployment => {
        if (deployment.version == null) {
          return null;
        }
        const info: RemoteBundleInfo = {
          name: deployment.name,
          version: deployment.version,
        };
        return info;
      })
      .filter(x => x != null);
    return c.json(bundles);
  });

  app.get('/bundles/:name', async c => {
    const bundleName = c.req.param('name');
    const deployment = await getBundleDeployment(c.env, bundleName);
    if (deployment == null || deployment.version == null) {
      return c.notFound();
    }
    const info: RemoteBundleInfo = {
      name: deployment.name,
      version: deployment.version,
    };
    return c.json(info);
  });

  app.get('/bundles/:name/download', async c => {
    const bundleName = c.req.param('name');
    const deployment = await getBundleDeployment(c.env, bundleName);
    if (deployment == null || deployment.version == null) {
      return c.notFound();
    }
    const resp = await getBundleDataResponse(c.env, bundleName, deployment.version);
    if (resp == null) {
      return c.notFound();
    }
    return resp;
  });

  app.get('/bundles/:name/download/:version', async c => {
    if (allowOnlyLatest) {
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

  app.onError((e, c) => {
    if (typeof (e as HTTPResponseError).getResponse === 'function') {
      return (e as HTTPResponseError).getResponse();
    }
    if (e instanceof ZodError) {
      return c.json(
        {
          message: `validation failed: ${e.message}`,
          issues: e.issues,
        },
        { status: 400 }
      );
    }
    return c.json({ message: e.message }, { status: 500 });
  });

  return app;
}

export const wvbRemote = webviewBundleRemote;
