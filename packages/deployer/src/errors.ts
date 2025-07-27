import type { DeployInfo, UploadReleaseInfo } from './types.js';

export class ReleaseAlreadyUploadedError extends Error {
  constructor({ name, version, channel }: Pick<UploadReleaseInfo, 'name' | 'version' | 'channel'>) {
    super(
      `bundle "${name}" version "${version}" has already been released.${channel != null ? ` (channel: ${channel})` : ''}`
    );
    this.name = 'ReleaseAlreadyUploadedError';
  }
}

export class ReleaseNotFoundError extends Error {
  constructor({ name, version, channel }: Pick<DeployInfo, 'name' | 'version' | 'channel'>) {
    super(`bundle "${name}" version "${version}" not found.${channel != null ? ` (channel: ${channel})` : ''}`);
    this.name = 'ReleaseNotFoundError';
  }
}

export class UnexpectedError<E = unknown> extends Error {
  constructor(
    message: string,
    public readonly originError: E
  ) {
    super();
    this.name = 'UnexpectedError';
    this.message =
      originError instanceof Error ? `${message} (origin: ${originError.name}, ${originError.message})` : message;
  }
}
