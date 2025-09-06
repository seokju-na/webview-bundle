import { Cli } from 'clipanion';
import pkgJson from '../package.json' with { type: 'json' };
import { CreateCommand } from './commands/create.js';
import { ExtractCommand } from './commands/extract.js';

const [, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'webview-bundle-cli',
  binaryName: app,
  binaryVersion: pkgJson.version,
});

cli.register(CreateCommand);
cli.register(ExtractCommand);
cli.runExit(args);
