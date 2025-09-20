#!/usr/bin/env -S node --no-warnings=ExperimentalWarning --experimental-strip-types
import { Cli } from 'clipanion';
import { ArtifactsMergeCommand } from './commands/artifacts-merge.ts';
import { ArtifactsNapiCommand } from './commands/artifacts-napi.ts';
import { ArtifactsSpreadCommand } from './commands/artifacts-spread.ts';
import { AttwCommand } from './commands/attw.ts';
import { Release } from './commands/release.ts';

const [node, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'xtask',
  binaryName: `${node} ${app}`,
  binaryVersion: '0.0.0',
});

cli.register(Release);
cli.register(ArtifactsSpreadCommand);
cli.register(ArtifactsMergeCommand);
cli.register(ArtifactsNapiCommand);
cli.register(AttwCommand);
cli.runExit(args);
