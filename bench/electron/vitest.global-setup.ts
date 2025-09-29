import { buildAllFixtures } from '@bench/tools';
import { build } from 'esbuild';

const external = ['electron', '@webview-bundle/electron-binding'];

export default async function setup(): Promise<void> {
  await buildAllFixtures();
  await Promise.all([
    build({
      entryPoints: ['./apps/fs/main.ts'],
      outfile: './dist/fs/main.mjs',
      format: 'esm',
      target: 'node20',
      platform: 'node',
      bundle: true,
      external,
    }),
    build({
      entryPoints: ['./apps/wvb/main.ts'],
      outfile: './dist/wvb/main.mjs',
      format: 'esm',
      target: 'node20',
      platform: 'node',
      bundle: true,
      external,
    }),
  ]);
}
