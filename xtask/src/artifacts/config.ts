import fs from 'node:fs/promises';
import * as workspaceUtils from '../workspaceUtils';

export interface ArtifactsFile {
  source: string;
  dist: string;
}

interface ArtifactsConfigRaw {
  dir: string;
  files: ArtifactsFile[];
}

export class ArtifactsConfig {
  readonly dir: string;
  readonly absoluteDir: string;
  readonly files: ArtifactsFile[];

  static async load(filepath: string) {
    const raw: ArtifactsConfigRaw = JSON.parse(await fs.readFile(filepath, 'utf8'));
    return new ArtifactsConfig(raw);
  }

  protected constructor(raw: ArtifactsConfigRaw) {
    this.dir = raw.dir;
    this.absoluteDir = workspaceUtils.absolutePathFromRootDir(raw.dir);
    this.files = raw.files;
  }
}
