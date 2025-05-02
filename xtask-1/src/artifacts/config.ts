import fs from 'node:fs/promises';
import { z } from 'zod';
import * as workspaceUtils from '../workspaceUtils';

export const ArtifactFileSchema = z.object({
  source: z.string().describe('Artifact file source path or glob pattern to upload.'),
  dist: z.string().describe('Destination directory to located artifacts files after downloaded.'),
});
export type ArtifactFile = z.infer<typeof ArtifactFileSchema>;

export const ArtifactsConfigSchema = z.object({
  dir: z
    .string()
    .describe(
      'Directory to merge all of artifacts files. After uploading this directory specified in CI, you can use it later in the release stage, etc.'
    ),
  files: ArtifactFileSchema.array().describe('Describe artifact files.'),
});
type ArtifactsConfigRaw = z.infer<typeof ArtifactsConfigSchema>;

export class ArtifactsConfig {
  readonly dir: string;
  readonly absoluteDir: string;
  readonly files: ArtifactFile[];

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
