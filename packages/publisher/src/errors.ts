import type { PublishOptions } from './publisher.js';

export class AlreadyPublishedError extends Error {
  readonly name = 'AlreadyPublishedError';

  constructor(public readonly options: PublishOptions) {
    super(`${options.name} webview bundle already published.`);
  }
}

export function isAlreadyPublishedError(e: unknown): e is AlreadyPublishedError {
  return e != null && typeof e === 'object' && (e as any)?.name === 'AlreadyPublishedError';
}
