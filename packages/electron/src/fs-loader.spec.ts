import path from 'node:path';
import { create, encode } from '@webview-bundle/node-binding';
import { type PortablePath, xfs } from '@yarnpkg/fslib';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { FSLoader } from './fs-loader.js';
import { URI } from './uri.js';

describe('FSLoader', () => {
  let tmpdir: string;

  beforeEach(async () => {
    tmpdir = await xfs.mktempPromise();
  });

  afterEach(async () => {
    await xfs.rmtempPromise();
  });

  it('load bundle from file system', async () => {
    const loader = new FSLoader({
      resolveFilepath(uri) {
        return path.join(tmpdir, `${uri.host}.wvb`);
      },
      fs: {
        readFile: path => xfs.readFilePromise(path as PortablePath),
      },
    });
    const bundle = await create([
      { path: 'index.html', data: Buffer.from('<h1>Hello World</h1>', 'utf8') },
      { path: 'index.js', data: Buffer.from('console.log("Hello World");', 'utf8') },
    ]);
    const buf = await encode(bundle);
    await xfs.writeFilePromise(path.join(tmpdir, 'main.wvb') as PortablePath, buf);
    const loaded = await loader.load(new URI('app://main'));
    await expect(loaded.readFile('index.html')).resolves.toEqual(Buffer.from('<h1>Hello World</h1>', 'utf8'));
    await expect(loaded.readFile('index.js')).resolves.toEqual(Buffer.from('console.log("Hello World");', 'utf8'));
  });
});
