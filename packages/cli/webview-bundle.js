#!/usr/bin/env node

const cli = require('./index');

const [, , ...args] = process.argv;
cli.run(args).catch(e => {
  if (e.code !== 'InvalidArg') {
    console.error(e.message);
  }
  process.exit(1);
});
