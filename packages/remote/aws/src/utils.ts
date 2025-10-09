import { NotFound } from '@aws-sdk/client-s3';

export function isNotFoundError(e: unknown): boolean {
  return e instanceof NotFound || (e != null && typeof e === 'object' && (e as any).name === 'NotFound');
}
