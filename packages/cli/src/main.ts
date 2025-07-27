import { Cli } from 'clipanion';
import pkgJson from '../package.json' with { type: 'json' };
import { PackCommand } from './commands/pack.js';

const [, app, ...args] = process.argv;

const cli = new Cli({
  binaryLabel: 'webview-bundle-cli',
  binaryName: app,
  binaryVersion: pkgJson.version,
});

cli.register(PackCommand);
cli.runExit(args);
