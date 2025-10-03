import path from 'node:path';
import { type BundleSource, initBundleSource } from '@webview-bundle/electron/binding';
import { app } from 'electron';

let globalSource: Promise<BundleSource> | null = null;

export function getSource(): Promise<BundleSource> {
  if (globalSource == null) {
    throw new Error('Cannot get bundle source. Please make sure you have initialized the webview bundle.');
  }
  return globalSource;
}

export interface SourceOptions {
  builtinDir?: string;
  remoteDir?: string;
}

export function initSource(options: SourceOptions = {}): Promise<BundleSource> {
  if (globalSource != null) {
    return globalSource;
  }
  const { builtinDir = defaultBuiltinDir(), remoteDir = defaultRemoteDir() } = options;
  globalSource = initBundleSource(builtinDir, remoteDir);
  return globalSource;
}

function defaultBuiltinDir(): string {
  return path.join(app.isPackaged ? process.resourcesPath : process.cwd(), 'bundles');
}

function defaultRemoteDir(): string {
  return path.join(app.getPath('userData'), 'bundles');
}
