import { z } from 'zod/v4';

export const RemoteBundleStoreSchema = z.object({
  name: z.string(),
  version: z.string().optional(),
  channels: z.record(z.string(), z.string()).optional(),
});
export type RemoteBundleStore = z.infer<typeof RemoteBundleStoreSchema>;
