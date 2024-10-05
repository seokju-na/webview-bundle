import { readdirSync } from 'node:fs';
import path from 'node:path';

export function findRootDir() {
  let current = process.cwd();
  while (true) {
    const files = readdirSync(current);
    if (files.some(x => x === 'yarn.lock')) {
      break;
    }
    current = path.resolve(current, '../');
  }
  return current;
}
