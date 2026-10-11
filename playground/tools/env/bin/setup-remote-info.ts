#!/usr/bin/env node
import fs from 'node:fs/promises';
import path from 'node:path';
import { loadEnv } from '../src/index.ts';

const env = loadEnv();
await fs.writeFile(path.join(process.cwd(), 'remote.json'), JSON.stringify(env.remote, null, 2));
