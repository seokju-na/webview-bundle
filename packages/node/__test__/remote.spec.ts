import { Buffer } from 'node:buffer';
import { type ServerType, serve } from '@hono/node-server';
import getPort from 'get-port';
import { Hono } from 'hono';
import { afterAll, beforeAll, describe, expect, it } from 'vitest';
import { BundleBuilder, Remote, writeBundleIntoBuffer } from '../index.js';

let port: number;
let server: ServerType;
let allowOnlyLatest = false;

beforeAll(async () => {
  port = await getPort();
  const app = new Hono();

  function makeBundleResponse(bundleName: string, version: string) {
    const headers = new Headers();
    headers.set('content-type', 'application/webview-bundle');
    headers.set('webview-bundle-name', bundleName);
    headers.set('webview-bundle-version', version);
    const builder = new BundleBuilder();
    builder.insertEntry('/index.html', Buffer.from('<h1>Hello World</h1>', 'utf8'));
    const bundle = builder.build();
    const buf = writeBundleIntoBuffer(bundle);
    return new Response(new Uint8Array(buf), { status: 200, headers });
  }

  // GET /bundles
  app.get('/bundles', c => c.json(['bundle1']));
  // GET /bundles/{name}
  app.get('/bundles/:name', async c => {
    const bundleName = c.req.param('name');
    if (bundleName === 'bundle1') {
      return makeBundleResponse(bundleName, '1.0.0');
    }
    return c.notFound();
  });
  // GET /bundles/{name}/{version}
  app.get('/bundles/:name/:version', async c => {
    if (allowOnlyLatest) {
      return c.json({}, { status: 403 });
    }
    const bundleName = c.req.param('name');
    const version = c.req.param('version');
    if (bundleName === 'bundle1' && version === '1.0.0') {
      return makeBundleResponse(bundleName, version);
    }
    return c.notFound();
  });
  server = serve({ fetch: app.fetch, port });
});
afterAll(() => {
  allowOnlyLatest = false;
  return new Promise<void>((resolve, reject) => {
    if (server == null) {
      return;
    }
    server.close(e => {
      e != null ? reject(e) : resolve();
    });
  });
});

describe('remote', () => {
  it('list bundles', async () => {
    const remote = new Remote(`http://localhost:${port}`);
    await expect(remote.listBundles()).resolves.toEqual(['bundle1']);
  });

  it('get bundle info', async () => {
    const remote = new Remote(`http://localhost:${port}`);
    await expect(remote.getInfo('bundle1')).resolves.toEqual({
      name: 'bundle1',
      version: '1.0.0',
    });
  });

  it('download bundle', async () => {
    const remote = new Remote(`http://localhost:${port}`);
    const [info, bundle] = await remote.download('bundle1');
    expect(info).toEqual({ name: 'bundle1', version: '1.0.0' });
    expect(bundle.getData('/index.html')).toEqual(Buffer.from('<h1>Hello World</h1>', 'utf8'));
  });

  it('download bundle with specific version', async () => {
    const remote = new Remote(`http://localhost:${port}`);
    const [info, bundle] = await remote.downloadVersion('bundle1', '1.0.0');
    expect(info).toEqual({ name: 'bundle1', version: '1.0.0' });
    expect(bundle.getData('/index.html')).toEqual(Buffer.from('<h1>Hello World</h1>', 'utf8'));

    allowOnlyLatest = true;
    await expect(remote.downloadVersion('bundle1', '1.0.0')).rejects.toThrowError(/remote forbidden/);
  });

  it('bundle not found', async () => {
    const remote = new Remote(`http://localhost:${port}`);
    await expect(remote.download('not_found')).rejects.toThrowError(/bundle not found/);
  });
});
