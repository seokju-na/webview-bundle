import { readFile } from 'node:fs/promises';
import { z } from 'zod';
import * as workspaceUtils from '../workspaceUtils';

export const ReleaseScriptSchema = z.object({
  command: z.string().describe('Command to execute script within.'),
  cwd: z.string().describe('Current working directory which script will be executed.').optional(),
  env: z
    .record(z.string(), z.string())
    .describe('Environment variables to injected into script child process.')
    .optional(),
});
export type ReleaseScript = z.infer<typeof ReleaseScriptSchema>;

export const ReleasePackageSchema = z.object({
  versionedFiles: z
    .string()
    .array()
    .describe(
      'Version files (e.g. package.json, Cargo.toml) which version will be bumped when release process proceed.'
    ),
  changelog: z.string().describe('Changelog path file for this package.'),
  scopes: z
    .string()
    .array()
    .describe('Scopes to determine which git conventional commits are related to this package.')
    .optional(),
  beforeReleaseScripts: ReleaseScriptSchema.array().describe('Scripts to execute before release step.').optional(),
});
export type ReleasePackage = z.infer<typeof ReleasePackageSchema>;

export const ReleaseGitHubSchema = z.object({
  repo: z.object({
    owner: z.string().describe('GitHub repository owner name.'),
    name: z.string().describe('GitHub repository name.'),
  }),
});
export type ReleaseGitHub = z.infer<typeof ReleaseGitHubSchema>;

export const ReleaseConfigSchema = z.object({
  rootChangelog: z
    .string()
    .describe('Changelog file path which describes all of crates/packages changelog.')
    .optional(),
  packages: z.record(z.string(), ReleasePackageSchema).describe('Packages to execute release steps.'),
  github: ReleaseGitHubSchema,
});
type ReleaseConfigRaw = z.infer<typeof ReleaseConfigSchema>;

export class ReleaseConfig {
  readonly rootChangelog?: string;
  readonly packages = new Map<string, ReleasePackage>();
  readonly github: ReleaseGitHub;

  static async load(filepath: string) {
    const raw: ReleaseConfigRaw = JSON.parse(await readFile(filepath, 'utf8'));
    return new ReleaseConfig(parseConfigRaw(raw));
  }

  protected constructor(raw: ReleaseConfigRaw) {
    this.rootChangelog =
      raw.rootChangelog != null ? workspaceUtils.absolutePathFromRootDir(raw.rootChangelog) : undefined;
    this.github = raw.github;
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
