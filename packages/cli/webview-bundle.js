#!/usr/bin/env node

import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);
const cli = require('./binding.cjs');

const [, , ...args] = process.argv;
cli.run(args).catch(e => {
  if (e.code !== 'InvalidArg') {
    console.error(e.message);
  }
  process.exit(1);
});
