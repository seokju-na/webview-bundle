#!/usr/bin/env -S node --no-warnings=ExperimentalWarning --experimental-strip-types
import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { toJSONSchema } from 'zod/v4';
import { ConfigSchema } from '../config.ts';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.join(dirname, '..', '..');

const schema = toJSONSchema(ConfigSchema);
await fs.writeFile(path.join(rootDir, 'xtask.$schema.json'), JSON.stringify(schema, null, 2));
