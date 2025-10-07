import { z } from 'zod/v4';

export const RemoteBundleInfoSchema = z.object({
  name: z.string(),
  version: z.string(),
  integrity: z.string().optional(),
});
