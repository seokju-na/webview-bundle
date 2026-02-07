import { Cli } from 'clipanion';
import pkg from '../package.json' with { type: 'json' };
import { BuiltinCommand } from './commands/builtin.js';
import { CreateCommand } from './commands/create.js';
import { ExtractCommand } from './commands/extract.js';
import { RemoteCurrentCommand } from './commands/remote/current.js';
import { RemoteDeployCommand } from './commands/remote/deploy.js';
import { RemoteDownloadCommand } from './commands/remote/download.js';
import { RemoteListCommand } from './commands/remote/list.js';
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
cli.register(RemoteCurrentCommand);
cli.register(RemoteListCommand);
cli.register(RemoteUploadCommand);
cli.register(RemoteDeployCommand);
cli.register(RemoteDownloadCommand);
cli.register(BuiltinCommand);
cli.runExit(args);
