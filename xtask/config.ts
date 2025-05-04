import fs from 'node:fs/promises';
import path from 'node:path';
import { z } from 'zod';
import { ROOT_DIR } from './consts.ts';

export const ScriptConfigSchema = z.object({
  command: z.string(),
  args: z.string().array().optional(),
  cwd: z.string().optional(),
});
export type ScriptConfig = z.infer<typeof ScriptConfigSchema>;

export const PackageConfigSchema = z.object({
  versionedFiles: z.string().array(),
  changelog: z.string().optional(),
  scopes: z.string().array(),
  beforePublishScripts: ScriptConfigSchema.array().optional(),
});
export type PackageConfig = z.infer<typeof PackageConfigSchema>;

export const GitHubRepoConfigSchema = z.object({
  owner: z.string(),
  name: z.string(),
});
export type GitHubRepoConfig = z.infer<typeof GitHubRepoConfigSchema>;

export const GitHubConfigSchema = z.object({
  repo: GitHubRepoConfigSchema,
});
export type GitHubConfig = z.infer<typeof GitHubConfigSchema>;

export const ArtifactFileConfigSchema = z.object({
  source: z.string(),
  dist: z.string(),
});
export type ArtifactFileConfig = z.infer<typeof ArtifactFileConfigSchema>;

export const ArtifactConfigSchema = z.object({
  dir: z.string(),
  files: ArtifactFileConfigSchema.array(),
});
export type ArtifactConfig = z.infer<typeof ArtifactConfigSchema>;

export const ConfigSchema = z.object({
  rootChangelog: z.string().optional(),
  packages: z.record(z.string(), PackageConfigSchema),
  github: GitHubConfigSchema,
  artifacts: ArtifactConfigSchema.optional(),
});
export type Config = z.infer<typeof ConfigSchema>;

export async function loadConfig(filepath: string): Promise<Config> {
  const content = await fs.readFile(path.join(ROOT_DIR, filepath), 'utf8');
  return ConfigSchema.parse(JSON.parse(content));
}
