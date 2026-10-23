import path from 'node:path';
import type { Source, SourceConfig } from '@wvb/node';
import { app } from 'electron';
import { wvbNode } from './native.js';

/**
 * Bundle source configuration with Electron-specific directory defaults.
 *
 * Builtin bundles default to `bundles` beside the app resources; downloaded bundles default to
 * Electron's per-user data directory.
 */
export interface SourceOptions extends Omit<SourceConfig, 'builtinDir' | 'remoteDir'> {
  /** Directory containing bundles shipped with the application. */
  builtinDir?: string;
  /** Directory used for downloaded bundles and their manifest. */
  remoteDir?: string;
}

/** Creates a source using Electron's standard packaged and user-data directories. */
export function source(options: SourceOptions = {}): Source {
  const {
    builtinDir = defaultBuiltinDir(),
    remoteDir = defaultRemoteDir(),
    ...otherOptions
  } = options;
  return new wvbNode.Source({
    builtinDir,
    remoteDir,
    ...otherOptions,
  });
}

function defaultBuiltinDir(): string {
  if (app.isPackaged) {
    return path.join(process.resourcesPath, 'bundles');
  }
  return path.join(process.cwd(), '.wvb', 'builtin', 'bundles');
}

function defaultRemoteDir(): string {
  return path.join(app.getPath('userData'), 'bundles');
}
