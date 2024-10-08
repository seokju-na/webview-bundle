import { readFile } from 'node:fs/promises';
import * as workspaceUtils from '../workspaceUtils';

interface ReleaseConfigRaw {
  rootChangelog?: string;
  packages: Record<string, ReleasePackage>;
}

export interface ReleaseScript {
  command: string;
  cwd?: string;
  env?: Record<string, string>;
}

export interface ReleasePackage {
  versionedFiles: string[];
  changelog: string;
  scopes?: string[];
  beforeReleaseScripts?: ReleaseScript[];
}

export class ReleaseConfig {
  readonly rootChangelog?: string;
  readonly packages = new Map<string, ReleasePackage>();

  static async load(filepath: string) {
    const raw: ReleaseConfigRaw = JSON.parse(await readFile(filepath, 'utf8'));
    return new ReleaseConfig(parseConfigRaw(raw));
  }

  protected constructor(raw: ReleaseConfigRaw) {
    this.rootChangelog =
      raw.rootChangelog != null ? workspaceUtils.absolutePathFromRootDir(raw.rootChangelog) : undefined;
    for (const [name, pkg] of Object.entries(raw.packages ?? {})) {
      this.packages.set(name, pkg);
    }
  }
}

function parseConfigRaw(raw: ReleaseConfigRaw) {
  raw.packages = Object.fromEntries(
    Object.entries(raw.packages).map(([name, pkg]) => [
      name,
      {
        ...pkg,
        versionedFiles: pkg.versionedFiles.map(workspaceUtils.absolutePathFromRootDir),
        changelog: workspaceUtils.absolutePathFromRootDir(pkg.changelog),
      },
    ])
  );
  return raw;
}
