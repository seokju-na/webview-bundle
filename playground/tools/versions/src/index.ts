import fs from 'node:fs';
import path from 'node:path';

export function listVersions(cwd = process.cwd()): string[] {
  const versionsDir = path.join(cwd, 'versions');
  const dirs = fs.readdirSync(versionsDir, { withFileTypes: true });
  const versions = dirs.filter(x => x.isDirectory()).map(x => x.name);

  return versions;
}

export function versionRootDir(version: string, cwd = process.cwd()): string {
  return path.join(cwd, 'versions', version);
}

export function versionDistDir(version: string, cwd = process.cwd()): string {
  return path.join(cwd, 'dist', 'versions', version);
}

export function getPickedVersion(cwd = process.cwd()): string {
  const { VERSION: version } = process.env;

  const versions = listVersions(cwd);
  const pickedVersion = versions.find(x => x === version);

  if (pickedVersion == null) {
    throw new Error(`cannot pick version: ${pickedVersion} (candidates: ${versions.join(', ')})`);
  }

  return pickedVersion;
}
