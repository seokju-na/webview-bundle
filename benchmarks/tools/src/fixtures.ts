import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { execa } from 'execa';
import { Listr } from 'listr2';
import glob from 'tiny-glob';

const __dirname = fileURLToPath(import.meta.url);
const currentPkgDir = path.resolve(__dirname, '../');
const fixturesDir = path.resolve(__dirname, '../../../fixtures');

export interface FixtureInfo {
  name: string;
  dir: string;
}

export function listFixtures() {
  const names = fs.readdirSync(fixturesDir);
  const list: FixtureInfo[] = [];
  for (const name of names) {
    const dir = path.join(fixturesDir, name);
    const stat = fs.statSync(dir);
    if (stat.isDirectory()) {
      list.push({ name, dir });
    }
  }
  return list;
}

export function getFixtureDir(name: string) {
  return path.join(fixturesDir, name);
}

export function getFixtureOutDir(name: string) {
  return path.join(getFixtureDir(name), 'out');
}

export function getFixtureOutWebviewBundleFilePath(name: string) {
  return path.join(getFixtureDir(name), 'out.wvb');
}

export interface FixtureFile {
  path: string;
  absolutePath: string;
}

export async function findAllFixtureFiles(name: string) {
  const outDir = getFixtureOutDir(name);
  const paths = await glob('**/*', {
    cwd: getFixtureOutDir(name),
    dot: true,
    absolute: false,
    filesOnly: true,
  });
  const files = paths.map((filePath): FixtureFile => {
    return {
      path: filePath,
      absolutePath: path.join(outDir, filePath),
    };
  });
  return files;
}

export async function buildFixture(name: string): Promise<boolean> {
  await execa('yarn', ['out'], { cwd: getFixtureDir(name) });
  await execa(
    'yarn',
    ['webview-bundle', 'pack', getFixtureOutDir(name), '-o', getFixtureOutWebviewBundleFilePath(name), '--truncate'],
    { cwd: currentPkgDir }
  );
  return true;
}

export async function buildAllFixtures() {
  const fixtures = listFixtures();
  const task = new Listr(
    fixtures.map(fixture => ({
      title: `Building "${fixture.name}"...`,
      task: async () => {
        await buildFixture(fixture.name);
      },
    }))
  );
  await task.run();
}
