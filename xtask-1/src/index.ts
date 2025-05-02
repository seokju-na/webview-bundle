import { Cli } from 'clipanion';
import { ArtifactsMergeCommand } from './artifacts/ArtifactsMergeCommand';
import { ArtifactsSpreadCommand } from './artifacts/ArtifactsSpreadCommand';
import { BuildJSLibraryCommand } from './build/BuildJSLibraryCommand';
import { PrepareReleaseCommand } from './release/PrepareReleaseCommand';
import { PrepareReleasePRCommand } from './release/PrepareReleasePRCommand';
import { ReleaseCommand } from './release/ReleaseCommand';

const [node, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'xtask',
  binaryName: `${node} ${app}`,
  binaryVersion: '0.0.0',
});

cli.register(ReleaseCommand);
cli.register(PrepareReleaseCommand);
cli.register(PrepareReleasePRCommand);
cli.register(ArtifactsMergeCommand);
cli.register(ArtifactsSpreadCommand);
cli.register(BuildJSLibraryCommand);
cli.runExit(args);
