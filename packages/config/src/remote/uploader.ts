import type { Buffer } from 'node:buffer';

export interface RemoteUploadParams {
  bundle: Buffer;
  bundleName: string;
  version: string;
  force?: boolean;
  integrity?: string;
  signature?: string;
}

export interface RemoteUploadProgress {
  /** Number of bytes successfully transferred so far */
  loaded?: number;
  /** Total payload size in byres */
  total?: number;
  /** 1-based multipart part index currently being uploaded */
  part?: number;
}

export interface BaseRemoteUploader {
  _onUploadProgress?: (progress: RemoteUploadProgress) => void;
  upload(params: RemoteUploadParams): Promise<void>;
}
