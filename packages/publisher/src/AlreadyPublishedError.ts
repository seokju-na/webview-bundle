import type { PublishOptions } from './PublishOptions';

export class AlreadyPublishedError extends Error {
  readonly name = 'AlreadyPublishedError';

  constructor(public readonly options: PublishOptions) {
    super(`${options.name} webview bundle already published.`);
  }
}
