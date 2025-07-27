import type { Buffer } from 'node:buffer';

export type BundleData =
  | string
  | Buffer
  | Uint8Array
  | number[]
  | Buffer<ArrayBufferLike>
  | Uint8Array<ArrayBufferLike>;

export interface UploadReleaseInfo {
  name: string;
  bundle: BundleData;
  version: string;
  channel?: string;
  force?: boolean;
}

export interface DeployInfo {
  name: string;
  version: string;
  channel?: string;
}
