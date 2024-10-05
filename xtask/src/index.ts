import { Cli } from 'clipanion';
import { ReleaseCommand } from './release/ReleaseCommand';

const [node, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'xtask',
  binaryName: `${node} ${app}`,
  binaryVersion: '0.0.0',
});

cli.register(ReleaseCommand);
cli.runExit(args);
