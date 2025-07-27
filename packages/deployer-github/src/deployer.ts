import {
  Deployer,
  type DeployInfo,
  ReleaseAlreadyUploadedError,
  ReleaseNotFoundError,
  UnexpectedError,
  type UploadReleaseInfo,
} from '@webview-bundle/deployer';
import type { GitHubDeployerConfig, GitHubTagResolver } from './config.js';
import * as github from './github.js';
import { type GitHubRelease, isGitHubError } from './github.js';

export class GitHubDeployer extends Deployer {
  readonly name = 'github';

  constructor(public readonly config: GitHubDeployerConfig) {
    super();
  }

  async uploadRelease(info: UploadReleaseInfo): Promise<void> {
    const { name, bundle, version, force = false } = info;
    const { repo, generateReleaseNotes = false, tagResolver = defaultGitHubTagResolver, versionFile } = this.config;
    const tag = tagResolver(info);
    const client = this.getGitHubClient();
    let release: GitHubRelease | undefined;
    try {
      const resp = await client.repos.getReleaseByTag({
        owner: repo.owner,
        repo: repo.name,
        tag,
      });
      release = resp.data;
    } catch {
      const resp = await client.repos.createRelease({
        owner: repo.owner,
        repo: repo.name,
        tag_name: tag,
        name: tag,
        generateReleaseNotes,
        make_latest: 'false',
      });
      release = resp.data;
    }
    const exists = release.assets.find(x => x.name === 'bundle.wvb');
    if (exists != null) {
      if (force) {
        await client.repos.deleteReleaseAsset({
          owner: repo.owner,
          repo: repo.name,
          asset_id: exists.id,
        });
      } else {
        throw new ReleaseAlreadyUploadedError({ name, version });
      }
    }
    const bundleData = await this.writeBundleVersion(bundle, versionFile, version);
    try {
      await client.repos.uploadReleaseAsset({
        owner: repo.owner,
        repo: repo.name,
        release_id: release.id,
        url: release.upload_url,
        // https://github.com/octokit/rest.js/issues/1645
        data: bundleData as any,
        headers: {
          'content-type': 'application/webview-bundle',
          'content-length': Buffer.byteLength(bundleData),
        },
        name: 'bundle.wvb',
      });
    } catch (e) {
      throw new UnexpectedError('fail to upload release', e);
    }
  }

  async deploy(info: DeployInfo): Promise<void> {
    const { repo, tagResolver = defaultGitHubTagResolver } = this.config;
    const client = this.getGitHubClient();
    const tag = tagResolver(info);
    let release: GitHubRelease;
    try {
      const resp = await client.repos.getReleaseByTag({
        owner: repo.owner,
        repo: repo.name,
        tag,
      });
      release = resp.data;
    } catch (e) {
      if (isGitHubError(e) && e.status === 404) {
        throw new ReleaseNotFoundError(info);
      }
      throw new UnexpectedError('fail to get github release', e);
    }
    try {
      await client.repos.updateRelease({
        owner: repo.owner,
        repo: repo.name,
        release_id: release.id,
        make_latest: 'true',
      });
    } catch (e) {
      throw new UnexpectedError('fail to update release', e);
    }
  }

  private getGitHubClient() {
    const { authToken, clientOptions } = this.config;
    const client = github.client({
      auth: authToken,
      ...clientOptions,
    });
    return client;
  }
}

export const defaultGitHubTagResolver: GitHubTagResolver = info => {
  const { name, version, channel } = info;
  if (channel != null) {
    return `${name}/${version}/${channel}`;
  }
  return `${name}/${version}`;
};
