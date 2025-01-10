import path from 'node:path';
import type { OctokitOptions } from '@octokit/core';
import { retry } from '@octokit/plugin-retry';
import { Octokit } from '@octokit/rest';
import type { GetResponseDataTypeFromEndpointMethod } from '@octokit/types';
import { version } from '../package.json';

export type GitHubClientOptions = OctokitOptions;
export type GitHubClient = Octokit;
export type GitHubRelease = GetResponseDataTypeFromEndpointMethod<Octokit['rest']['repos']['getRelease']>;

export function client(options: OctokitOptions = {}): GitHubClient {
  const RetryableOctokit = Octokit.plugin(retry);
  const client = new RetryableOctokit({
    userAgent: `Webview Bundle/v${version}`,
    ...options,
  });
  return client;
}

// Based on https://github.com/electron/forge/blob/cbb64160bd48edb9fb3b7acba07a710b00cb477b/packages/publisher/github/src/util/github.ts#L51C1-L52C1
export function normalizeArtifactName(name: string) {
  return (
    path
      .basename(name)
      // Remove diacritics (e.g. é -> e)
      .normalize('NFD')
      .replace(/\p{Diacritic}/gu, '')
      // Replace special characters with dot
      .replace(/[^\w_.@+-]+/g, '.')
      // Replace multiple dots with a single dot
      .replace(/\.+/g, '.')
      // Remove leading dot if present
      .replace(/^\./g, '')
      // Remove trailing dot if present
      .replace(/\.$/g, '')
  );
}
