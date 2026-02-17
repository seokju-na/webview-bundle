import { S3Client } from '@aws-sdk/client-s3';
import { Hono, type Context as HonoContext } from 'hono';
import { getBundleDeployment } from './operations/getBundleDeployment.js';
import { getBundleDownloadPath } from './operations/getBundleDownloadPath.js';
import { listAllBundleDeployments } from './operations/listAllBundleDeployments.js';
import { type Bindings, type Context, getRemoteBundleDeploymentVersion } from './types.js';

export type WebviewBundleRemote = Hono<{ Bindings: Bindings; Variables: Context }>;

export interface WebviewBundleRemoteConfig {
  bucketName: string;
  region?: string;
  s3Client?: S3Client;
  /** Option to allow downloading other version instead of deployed version */
  allowOtherVersions?: boolean;
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
    const channel = c.req.query('channel');
    const deployments = await listAllBundleDeployments(getContext(c));
    const bundles = deployments
      .map(x => {
        const version = getRemoteBundleDeploymentVersion(x, channel);
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
    const deployment = await getBundleDeployment(getContext(c), bundleName);
    const version =
      channel != null
        ? (deployment?.channels?.[channel] ?? deployment?.version)
        : deployment?.version;
    if (deployment == null || version == null) {
      return c.notFound();
    }
    const request = c.env.request;
    request.uri = `/${getBundleDownloadPath(bundleName, version)}`;
    request.headers['x-webview-bundle'] = [{ key: 'X-Webview-Bundle', value: bundleName }];
    c.env.callback(null, request);
  });

  app.get('/bundles/:name/:version', async c => {
    if (config.allowOtherVersions !== true) {
      return new Response(null, { status: 403 });
    }
    const bundleName = c.req.param('name');
    const version = c.req.param('version');
    const request = c.env.request;
    request.uri = `/${getBundleDownloadPath(bundleName, version)}`;
    request.headers['x-webview-bundle'] = [{ key: 'X-Webview-Bundle', value: bundleName }];
    c.env.callback(null, request);
  });

  return app;
}

export const wvbRemote = webviewBundleRemote;
