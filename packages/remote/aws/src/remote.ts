import { S3Client } from '@aws-sdk/client-s3';
import { Hono, type Context as HonoContext } from 'hono';
import type { HTTPResponseError } from 'hono/types';
import { ZodError } from 'zod/v4';
import { getBundleDeployment } from './operations/getBundleDeployment.js';
import { getBundleDownloadPath } from './operations/getBundleDownloadPath.js';
import { listAllBundles } from './operations/listAllBundles.js';
import type { Bindings, Context } from './types.js';

export type WebviewBundleRemote = Hono<{ Bindings: Bindings; Variables: Context }>;

export interface WebviewBundleRemoteConfig {
  bucketName: string;
  region?: string;
  s3Client?: S3Client;
  allowOnlyLatest?: boolean;
}

export function webviewBundleRemote(config: WebviewBundleRemoteConfig): WebviewBundleRemote {
  const app = new Hono<{ Bindings: Bindings; Variables: Context }>();

  function getContext(c: HonoContext<{ Bindings: Bindings; Variables: Context }>): Context {
    const bucketName = c.get('bucketName');
    const s3Client = c.get('s3Client');
    return { bucketName, s3Client };
  }

  app.use(async (c, next) => {
    const { bucketName, s3Client, region } = config;
    c.set('bucketName', bucketName);
    c.set('s3Client', s3Client ?? new S3Client({ region }));
    await next();
  });

  app.get('/bundles', async c => {
    const bundles = await listAllBundles(getContext(c));
    return c.json(bundles);
  });

  app.get('/bundles/:name', async c => {
    const bundleName = c.req.param('name');
    const deployment = await getBundleDeployment(getContext(c), bundleName);
    if (deployment == null || deployment.version == null) {
      return c.notFound();
    }
    const request = c.env.request;
    request.uri = `/${getBundleDownloadPath(bundleName, deployment.version)}`;
    request.headers['x-webview-bundle'] = [{ key: 'X-Webview-Bundle', value: bundleName }];
    c.env.callback(null, request);
  });

  app.get('/bundles/:name/:version', async c => {
    if (config.allowOnlyLatest === true) {
      return new Response(null, { status: 403 });
    }
    const bundleName = c.req.param('name');
    const version = c.req.param('version');
    const request = c.env.request;
    request.uri = `/${getBundleDownloadPath(bundleName, version)}`;
    request.headers['x-webview-bundle'] = [{ key: 'X-Webview-Bundle', value: bundleName }];
    c.env.callback(null, request);
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
