import fs from 'node:fs/promises';
import path from 'node:path';
import { z } from 'zod';
import { ROOT_DIR } from './consts.ts';
import { BumpRuleSchema } from './version.ts';

export const StagedSchema = z.record(
  z.string(),
  z.object({
    commits: z.string().array(),
    bumpRule: BumpRuleSchema.optional(),
  })
);
export type Staged = z.infer<typeof StagedSchema>;

export async function loadStaged(filepath: string): Promise<Staged> {
  const content = await fs.readFile(path.isAbsolute(filepath) ? filepath : path.join(ROOT_DIR, filepath), 'utf8');
  return StagedSchema.parse(JSON.parse(content));
}

export async function saveStaged(filepath: string, staged: Staged): Promise<void> {
  await fs.writeFile(
    path.isAbsolute(filepath) ? filepath : path.join(ROOT_DIR, filepath),
    `${JSON.stringify(staged, null, 2)}\n`,
    'utf8'
  );
}
