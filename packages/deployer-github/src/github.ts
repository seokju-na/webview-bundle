import type { OctokitOptions } from '@octokit/core';
import { retry } from '@octokit/plugin-retry';
import { RequestError } from '@octokit/request-error';
import { Octokit } from '@octokit/rest';
import type { GetResponseDataTypeFromEndpointMethod } from '@octokit/types';

export type GitHubClientOptions = OctokitOptions;
export type GitHubClient = Octokit;
export type GitHubRelease = GetResponseDataTypeFromEndpointMethod<Octokit['rest']['repos']['getRelease']>;
export type GitHubError = RequestError;

export function client(options: OctokitOptions = {}): GitHubClient {
  const RetryableOctokit = Octokit.plugin(retry);
  const client = new RetryableOctokit({
    userAgent: 'Webview Bundle Publisher',
    ...options,
  });
  return client;
}

export function isGitHubError(e: unknown): e is GitHubError {
  return (
    e instanceof RequestError ||
    (e != null &&
      typeof e === 'object' &&
      (e as RequestError)?.name === 'HttpError' &&
      typeof (e as RequestError)?.status === 'number' &&
      'request' in e)
  );
}
