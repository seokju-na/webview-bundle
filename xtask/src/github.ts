import { Octokit } from '@octokit/rest';
import { memoize } from 'es-toolkit';

export const getGithub = memoize((auth: string) => {
  const api = new Octokit({
    auth,
    baseUrl: 'https://api.github.com',
  });
  return api;
});
