export type IpcErrorCode = 'BUNDLE_VERSION_NOT_FOUND';

export interface IpcErrorPayload {
  code: IpcErrorCode;
  message?: string;
}

export class IpcError extends Error {
  readonly code: IpcErrorCode;

  static from(code: IpcErrorCode, message?: string): IpcError {
    return new IpcError({ code, message });
  }

  constructor({ code, message }: IpcErrorPayload) {
    super(message != null ? `${code}: ${message}` : message);
    this.name = 'IpcError';
    this.code = code;
  }
}

export function isIpcError(e: unknown): e is IpcError {
  return e != null && typeof e === 'object' && (e as IpcError)?.name === 'IpcError';
}
