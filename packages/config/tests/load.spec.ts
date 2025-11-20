import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { execa } from 'execa';
import { afterAll, describe, it } from 'vitest';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.join(dirname, '..');

afterAll(async () => {
  await execa('yarn', ['pack'], { cwd: rootDir });
});

describe('yarn-pnp', () => {
  const fixtureDir = path.join(rootDir, 'tests', 'fixture', 'yarn-pnp');

  afterAll(async () => {
    await execa('yarn', { cwd: fixtureDir });
  });

  it('load "wvb.config.ts" from cjs', async () => {
    await execa('yarn', ['node', 'load.js', 'wvb.config.ts'], {
      cwd: fixtureDir,
    });
  });

  it('load "wvb.config.js" from cjs', async () => {
    await execa('yarn', ['node', 'load.js', 'wvb.config.js'], {
      cwd: fixtureDir,
    });
  });

  it('load "wvb.config.mjs" from cjs', async () => {
    await execa('yarn', ['node', 'load.js', 'wvb.config.mjs'], {
      cwd: fixtureDir,
    });
  });

  it('load "wvb.config.ts" from esm', async () => {
    await execa('yarn', ['node', 'load.mjs', 'wvb.config.ts'], {
      cwd: fixtureDir,
    });
  });

  it('load "wvb.config.js" from esm', async () => {
    await execa('yarn', ['node', 'load.mjs', 'wvb.config.js'], {
      cwd: fixtureDir,
    });
  });

  it('load "wvb.config.mjs" from esm', async () => {
    await execa('yarn', ['node', 'load.mjs', 'wvb.config.mjs'], {
      cwd: fixtureDir,
    });
  });
});
