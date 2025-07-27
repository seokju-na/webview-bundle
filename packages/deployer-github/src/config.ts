import type { GitHubClientOptions } from './github.js';

export type GitHubTagResolver = (options: { name: string; version: string; channel?: string }) => string;

export interface GitHubDeployerConfig {
  repo: {
    /**
     * The owner of your repository, this is either your username or the name of
     * the organization that owns the repository.
     */
    owner: string;
    /**
     * The name of your repository
     */
    name: string;
  };
  authToken?: string;
  /**
   * Whether to automatically generate release notes for the release
   */
  generateReleaseNotes?: boolean;
  tagResolver?: GitHubTagResolver;
  versionFile?: string;
  clientOptions?: GitHubClientOptions;
}
