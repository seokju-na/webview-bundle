import nodeFs from 'node:fs/promises';
import path from 'node:path';
import { type Bundle, decode } from '@webview-bundle/node-binding';
import { app } from 'electron';
import type { Loader } from './loader.js';
import type { URI } from './uri.js';

export interface FSLike {
  readFile(path: string, options?: unknown): Promise<Buffer>;
}

export interface FSLoaderConfig {
  resolveFilepath(uri: URI): string;
  fs?: FSLike | (() => FSLike) | Promise<FSLike>;
}

const defaultFS = (): FSLike => {
  return { readFile: nodeFs.readFile };
};

export class FSLoader implements Loader {
  private _fs: FSLike | null = null;

  static fromDir(dir: string, extraDir?: string[], options?: Omit<FSLoaderConfig, 'resolveFilepath'>) {
    const resolveFilepath = (uri: URI): string => {
      const filename = uri.host.endsWith('.wvb') ? uri.host : `${uri.host}.wvb`;
      if (extraDir != null && extraDir.length > 0) {
        return path.join(dir, ...extraDir, filename);
      }
      return path.join(dir, filename);
    };
    return new FSLoader({ ...options, resolveFilepath });
  }

  static fromHomeDir(extraDir?: string[], options?: Omit<FSLoaderConfig, 'resolveFilepath'>) {
    return FSLoader.fromDir(app.getPath('home'), extraDir, options);
  }

  static fromAppDataDir(extraDir?: string[], options?: Omit<FSLoaderConfig, 'resolveFilepath'>) {
    return FSLoader.fromDir(app.getPath('appData'), extraDir, options);
  }

  static fromUserDataDir(extraDir?: string[], options?: Omit<FSLoaderConfig, 'resolveFilepath'>) {
    return FSLoader.fromDir(app.getPath('userData'), extraDir, options);
  }

  constructor(private readonly config: FSLoaderConfig) {}

  getBundleName(uri: URI): string {
    const { resolveFilepath } = this.config;
    const filepath = resolveFilepath(uri);
    return path.basename(filepath);
  }

  async load(uri: URI): Promise<Bundle> {
    const { resolveFilepath } = this.config;
    const filepath = resolveFilepath(uri);
    const fs = await this.loadFS();
    const buf = await fs.readFile(filepath);
    const bundle = await decode(buf);
    return bundle;
  }

  private async loadFS() {
    if (this._fs != null) {
      return this._fs;
    }
    const { fs = defaultFS } = this.config;
    if (typeof fs === 'function') {
      this._fs = fs();
    } else {
      this._fs = await fs;
    }
    return this._fs;
  }
}
