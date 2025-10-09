import { z } from 'zod/v4';

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
