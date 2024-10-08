import { Cli } from 'clipanion';
import { PrepareReleaseCommand } from './release/PrepareReleaseCommand';
import { ReleaseCommand } from './release/ReleaseCommand';

const [node, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'xtask',
  binaryName: `${node} ${app}`,
  binaryVersion: '0.0.0',
});

cli.register(ReleaseCommand);
cli.register(PrepareReleaseCommand);
cli.runExit(args);
