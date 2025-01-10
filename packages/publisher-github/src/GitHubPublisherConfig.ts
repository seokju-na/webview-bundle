import type { GitHubClientOptions } from './github';

export interface GitHubPublisherConfig {
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
   * Whether this release should be tagged as a prerelease
   */
  prerelease?: boolean;
  /**
   * Whether this release should be tagged as a draft
   */
  draft?: boolean;
  /**
   * Prepended to the package version to determine the release name (default "v")
   */
  tagPrefix?: string;
  /**
   * Re-upload the new asset if you upload an asset with the same filename as existing asset
   */
  force?: boolean;
  /**
   * Whether to automatically generate release notes for the release
   */
  generateReleaseNotes?: boolean;
  clientOptions?: GitHubClientOptions;
}
