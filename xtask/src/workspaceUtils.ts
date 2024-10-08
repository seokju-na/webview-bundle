import { readdirSync } from 'node:fs';
import path from 'node:path';
import memoize from 'memoize';

export const findRootDir = memoize(() => {
  let dir = process.cwd();
  while (true) {
    const files = readdirSync(dir);
    if (files.some(x => x === 'yarn.lock')) {
      break;
    }
    dir = path.resolve(dir, '../');
  }
  return dir;
});

export function relativePathToRootDir(filepath: string) {
  return path.relative(findRootDir(), filepath);
}

export function absolutePathFromRootDir(filepath: string) {
  if (path.isAbsolute(filepath)) {
    return filepath;
  }
  const rootDir = findRootDir();
  return path.join(rootDir, filepath);
}
