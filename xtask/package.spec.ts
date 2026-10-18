import fs from 'node:fs/promises';
import path from 'node:path';
import { afterAll, beforeAll, beforeEach, describe, expect, it, vi } from 'vitest';
import { loadXtaskConfig } from './config.ts';
import { ROOT_DIR } from './consts.ts';
import { Package } from './package.ts';
import { Version } from './version.ts';

vi.mock('./consts.ts', async importOriginal => {
  const fs = await import('node:fs/promises');
  const os = await import('node:os');
  const path = await import('node:path');
  const original = await importOriginal<typeof import('./consts.ts')>();
  const root = await fs.mkdtemp(path.join(os.tmpdir(), 'wvb-package-test-'));
  await fs.copyFile(path.join(original.ROOT_DIR, 'taplo.toml'), path.join(root, 'taplo.toml'));
  return {
    ...original,
    ROOT_DIR: root,
  };
});

vi.mock('./config.ts', async importOriginal => ({
  ...(await importOriginal<typeof import('./config.ts')>()),
  loadXtaskConfig: vi.fn(),
}));

const fixtures = [
  ['packages/core', 'core'],
  ['packages/remote/local', 'remote-local'],
  ['packages/remote/providers/local', 'remote-providers-local'],
] as const;

beforeAll(async () => {
  for (const [dir, name] of fixtures) {
    const absoluteDir = path.join(ROOT_DIR, dir);
    await fs.mkdir(absoluteDir, { recursive: true });
    await fs.writeFile(
      path.join(absoluteDir, 'package.json'),
      JSON.stringify({ name: `@wvb/${name}`, version: '0.3.0' })
    );
  }
});

afterAll(async () => {
  await fs.rm(ROOT_DIR, { recursive: true, force: true });
});

beforeEach(() => {
  vi.mocked(loadXtaskConfig).mockResolvedValue({
    packages: ['packages/*', 'packages/remote/*', 'packages/remote/providers/*'],
  });
});

describe('Package.loadAll release names', () => {
  it.each(fixtures)('uses the full package path for %s', async (dir, name) => {
    const packages = await Package.loadAll();
    const pkg = packages.find(pkg => pkg.path === dir)!;
    expect(pkg.name).toBe(name);
    expect(pkg.versionFileNames).toEqual([`@wvb/${name}`]);
    expect(pkg.versionedGitTag.tagName).toBe(`${name}/0.3.0`);
    expect(pkg.versionedGitTag.tagRef).toBe(`refs/tags/${name}/0.3.0`);
    pkg.setVersion(Version.parse('0.4.0'));
    expect(pkg.nextVersionedGitTag.tagName).toBe(`${name}/0.4.0`);
  });

  it('uses the same default name for explicit package paths', async () => {
    vi.mocked(loadXtaskConfig).mockResolvedValue({
      packages: [{ path: 'packages/remote/local' }],
    });
    const [pkg] = await Package.loadAll();
    expect(pkg!.versionedGitTag.tagName).toBe('remote-local/0.3.0');
  });

  it('preserves an explicitly configured name when a glob also matches', async () => {
    vi.mocked(loadXtaskConfig).mockResolvedValue({
      packages: [{ path: 'packages/remote/local', name: 'custom-local' }, 'packages/remote/*'],
    });
    const [pkg] = await Package.loadAll();
    expect(pkg!.versionedGitTag.tagName).toBe('custom-local/0.3.0');
  });
});
