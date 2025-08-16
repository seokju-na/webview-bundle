export type UpdaterErrorCode =
  | 'REMOTES_FETCH_FAILED'
  | 'REMOTES_HTTP_ERROR'
  | 'REMOTES_INVALID_RESPONSE'
  | 'REMOTES_INVALID_URL';

export class UpdaterError extends Error {
  readonly code: UpdaterErrorCode;
  readonly origin?: unknown;

  constructor(code: UpdaterErrorCode, message?: string, origin?: unknown) {
    super(formatMessage(code, message, origin));
    this.name = 'WebViewBundleUpdaterError';
    this.code = code;
    this.origin = origin;
  }
}

function formatMessage(code: UpdaterErrorCode, message?: string, origin?: unknown): string {
  const msg = message ?? (origin as Error)?.message ?? '';
  return `[${code}] ${msg}`;
}
