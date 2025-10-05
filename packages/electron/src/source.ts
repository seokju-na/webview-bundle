import path from 'node:path';
import { BundleSource } from '@webview-bundle/electron/binding';
import { app } from 'electron';

export interface SourceOptions {
  builtinDir?: string;
  remoteDir?: string;
}

export function bundleSource(options: SourceOptions = {}): BundleSource {
  const { builtinDir = defaultBuiltinDir(), remoteDir = defaultRemoteDir() } = options;
  return new BundleSource(builtinDir, remoteDir);
}

function defaultBuiltinDir(): string {
  return path.join(app.isPackaged ? process.resourcesPath : process.cwd(), 'bundles');
}

function defaultRemoteDir(): string {
  return path.join(app.getPath('userData'), 'bundles');
}
