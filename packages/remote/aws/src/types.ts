import type { S3Client } from '@aws-sdk/client-s3';
import type { Callback, CloudFrontRequest } from 'hono/lambda-edge';
import { z } from 'zod/v4';

export interface Bindings {
  callback: Callback;
  request: CloudFrontRequest;
}

export interface Context {
  s3Client: S3Client;
  bucketName: string;
}

export const RemoteBundleDeploymentSchema = z.object({
  name: z.string(),
  version: z.string().optional(),
  channels: z.record(z.string(), z.string()).optional(),
});
export type RemoteBundleDeployment = z.infer<typeof RemoteBundleDeploymentSchema>;

export const RemoteBundleInfoSchema = z.object({
  name: z.string(),
  version: z.string(),
  integrity: z.string().optional(),
});
export type RemoteBundleInfo = z.infer<typeof RemoteBundleInfoSchema>;
