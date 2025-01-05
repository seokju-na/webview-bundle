import fs from 'node:fs/promises';
import { AlreadyPublishedError, type PublishOptions, Publisher } from '@webview-bundle/publisher';
import type { GitHubPublisherConfig } from './GitHubPublisherConfig';
import type { GitHubRelease } from './github';
import * as github from './github';

export class GitHubPublisher extends Publisher {
  readonly name = 'github';

  constructor(public readonly config: GitHubPublisherConfig) {
    super();
  }

  async publish(options: PublishOptions) {
    const { name, bundle, version } = options;
    const {
      repo,
      authToken,
      prerelease = false,
      draft = false,
      tagPrefix = 'v',
      force,
      generateReleaseNotes = false,
      clientOptions,
    } = this.config;
    const client = github.client({
      auth: authToken,
      ...clientOptions,
    });
    const tag = `${tagPrefix}${version}`;
    let release: GitHubRelease | null = null;
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
        draft,
        prerelease,
        generateReleaseNotes,
      });
      release = resp.data;
    }
    const assetName = github.normalizeArtifactName(name);
    const exists = release.assets.find(x => x.name === assetName);
    if (exists != null) {
      if (force) {
        await client.repos.deleteReleaseAsset({
          owner: repo.owner,
          repo: repo.name,
          asset_id: exists.id,
        });
      } else {
        throw new AlreadyPublishedError(options);
      }
    }
    await client.repos.uploadReleaseAsset({
      owner: repo.owner,
      repo: repo.name,
      release_id: release.id,
      url: release.upload_url,
      // https://github.com/octokit/rest.js/issues/1645
      data: (await fs.readFile(bundle)) as any,
      headers: {
        'content-type': 'application/webview-bundle',
        'content-length': (await fs.stat(bundle)).size,
      },
      name: assetName,
    });
  }
}
