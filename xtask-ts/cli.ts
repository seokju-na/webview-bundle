#!/usr/bin/env -S node --no-warnings=ExperimentalWarning --experimental-strip-types
import { Cli } from 'clipanion';
import { ReleaseCommand } from './commands/release-command.ts';

const [node, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'xtask',
  binaryName: `${node} ${app}`,
  binaryVersion: '0.0.0',
});

cli.register(ReleaseCommand);
cli.runExit(args);
