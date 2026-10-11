import { assertEquals, assertRejects } from '@std/assert';
import { join } from '@std/path';
import { install } from './install.ts';
import { releaseAssetName, sha256Hex } from './release.ts';

Deno.test('install keeps multiple architectures in the same output directory', async () => {
  const out = await Deno.makeTempDir({ prefix: 'wvb-install-' });
  const targets = ['aarch64-apple-darwin', 'x86_64-apple-darwin'];
  const assets = new Map(
    targets.map(target => [releaseAssetName(target), new TextEncoder().encode(target)])
  );
  const server = Deno.serve({ hostname: '127.0.0.1', port: 0, onListen: () => {} }, async req => {
    const name = new URL(req.url).pathname.slice(1);
    if (name.endsWith('.sha256')) {
      const asset = name.slice(0, -7);
      return new Response(`${await sha256Hex(assets.get(asset)!)}  ${asset}`);
    }
    return new Response(assets.get(name));
  });
  try {
    const paths = await Promise.all(
      targets.map(target =>
        install({
          out,
          target,
          url: `http://127.0.0.1:${server.addr.port}`,
        })
      )
    );
    for (const [index, target] of targets.entries()) {
      assertEquals(paths[index], join(out, target, 'libwvb_deno.dylib'));
      assertEquals(await Deno.readTextFile(paths[index]!), target);
    }
    await assertRejects(
      () => install({ out, target: '../invalid' }),
      Error,
      'invalid native library target'
    );
  } finally {
    await server.shutdown();
    await Deno.remove(out, { recursive: true });
  }
});
