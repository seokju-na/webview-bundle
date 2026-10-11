#!/usr/bin/env node
import { execa } from 'execa';
import { listVersions } from '../src/index.ts';

for (const version of listVersions()) {
  await execa('yarn', ['wvb', 'pack'], {
    env: {
      ...process.env,
      VERSION: version,
    },
    stdio: 'inherit',
  });
}
