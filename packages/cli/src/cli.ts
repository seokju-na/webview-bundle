import { Cli } from 'clipanion';
import pkgJson from '../package.json' with { type: 'json' };
import { CreateCommand } from './commands/create.js';
import { ExtractCommand } from './commands/extract.js';
import { RemoteS3UploadCommand } from './commands/remote-s3-uplaod.js';
import { ServeCommand } from './commands/serve.js';
import { TestCommand } from './commands/test.js';

const [, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'webview-bundle-cli',
  binaryName: app,
  binaryVersion: pkgJson.version,
});

cli.register(CreateCommand);
cli.register(ExtractCommand);
cli.register(ServeCommand);
cli.register(RemoteS3UploadCommand);
cli.register(TestCommand);
cli.runExit(args);
