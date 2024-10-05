import { readFile } from 'node:fs/promises';
import path from 'node:path';
import * as workspaceUtils from '../workspaceUtils';

interface ReleaseConfigRaw {
  packages?: Record<string, ReleasePackage>;
}

export interface ReleasePackage {
  versionedFiles: string[];
  scopes?: string[];
}

export class ReleaseConfig {
  readonly packages = new Map<string, ReleasePackage>();

  static async load(filepath: string) {
    const raw: ReleaseConfigRaw = JSON.parse(await readFile(filepath, 'utf8'));
    return new ReleaseConfig(parseConfigRaw(raw));
  }

  protected constructor(raw: ReleaseConfigRaw) {
    for (const [name, pkg] of Object.entries(raw.packages ?? {})) {
      this.packages.set(name, pkg);
    }
  }
}

function parseConfigRaw(raw: ReleaseConfigRaw) {
  const rootDir = workspaceUtils.findRootDir();
  if (raw.packages != null) {
    raw.packages = Object.fromEntries(
      Object.entries(raw.packages).map(([name, pkg]) => [
        name,
        {
          ...pkg,
          versionedFiles: pkg.versionedFiles.map(filepath => {
            if (path.isAbsolute(filepath)) {
              return filepath;
            }
            return path.join(rootDir, filepath);
          }),
        },
      ])
    );
  }
  return raw;
}
