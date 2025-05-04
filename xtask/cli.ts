#!/usr/bin/env -S node --no-warnings=ExperimentalWarning --experimental-strip-types
import { Cli } from 'clipanion';
import { ArtifactsMergeCommand } from './commands/artifacts-merge-command.ts';
import { ArtifactsSpreadCommand } from './commands/artifacts-spread-command.ts';
import { ReleaseCommand } from './commands/release-command.ts';
import { ArtifactsNapiCommand } from './commands/artifacts-napi-command.ts';

const [node, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'xtask',
  binaryName: `${node} ${app}`,
  binaryVersion: '0.0.0',
});

cli.register(ReleaseCommand);
cli.register(ArtifactsSpreadCommand);
cli.register(ArtifactsMergeCommand);
cli.register(ArtifactsNapiCommand);
cli.runExit(args);
