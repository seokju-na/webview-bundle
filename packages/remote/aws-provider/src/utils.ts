import { NoSuchKey } from '@aws-sdk/client-s3';

export function isNoSuchKeyError(e: unknown): boolean {
  return (
    e instanceof NoSuchKey ||
    (e != null && typeof e === 'object' && (e as any).name === 'NoSuchKey')
  );
}

export function toAWSHeaderName(headerName: string): string {
  return headerName.split('-').map(capitalize).join('-');
}

function capitalize(s: string): string {
  return s.charAt(0).toUpperCase() + s.slice(1).toLowerCase();
}
