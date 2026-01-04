import { Cli } from 'clipanion';
import pkg from '../package.json' with { type: 'json' };
import { CreateCommand } from './commands/create.js';
import { ExtractCommand } from './commands/extract.js';
import { RemoteUploadCommand } from './commands/remote/upload.js';
import { ServeCommand } from './commands/serve.js';

const [, , ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'webview-bundle-cli',
  binaryName: 'wvb',
  binaryVersion: pkg.version,
});

cli.register(CreateCommand);
cli.register(ExtractCommand);
cli.register(ServeCommand);
cli.register(RemoteUploadCommand);
cli.runExit(args);
