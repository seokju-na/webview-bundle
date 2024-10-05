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
