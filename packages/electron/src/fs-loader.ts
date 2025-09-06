import nodeFs from 'node:fs/promises';
import path from 'node:path';
import { app } from 'electron';
import { type Bundle, decode } from 'packages/node-binding-t';
import type { Loader } from './loader.js';

export interface FSLike {
  readFile(path: string, options?: unknown): Promise<Buffer>;
}

export interface FilePathResolverParams {
  baseDir: string;
  name: string;
}

export type FilePathResolver = (params: FilePathResolverParams) => string;

export const defaultFilePathResolver: FilePathResolver = ({ baseDir, name }) => {
  const filename = name.endsWith('.wvb') ? name : `${name}.wvb`;
  return path.join(baseDir, filename);
};

export interface FSLoaderOptions {
  filePathResolver?: FilePathResolver;
  fs?: FSLike | (() => FSLike) | Promise<FSLike>;
}

const defaultFS = (): FSLike => {
  return { readFile: nodeFs.readFile };
};

export class FSLoader implements Loader {
  private _fs: FSLike | null = null;
  private readonly options: FSLoaderOptions;

  static fromDir(dir: string, options?: FSLoaderOptions): FSLoader {
    return new FSLoader(dir, options);
  }

  static fromHomeDir(options?: FSLoaderOptions): FSLoader {
    return FSLoader.fromDir(app.getPath('home'), options);
  }

  static fromAppDataDir(options?: FSLoaderOptions): FSLoader {
    return FSLoader.fromDir(app.getPath('appData'), options);
  }

  static fromUserDataDir(options?: FSLoaderOptions): FSLoader {
    return FSLoader.fromDir(app.getPath('userData'), options);
  }

  constructor(
    public readonly baseDir: string,
    options?: FSLoaderOptions
  ) {
    this.options = options ?? {};
  }

  async load(name: string): Promise<Bundle> {
    const { filePathResolver = defaultFilePathResolver } = this.options;
    const fs = await this.loadFS();
    const filepath = filePathResolver({ baseDir: this.baseDir, name });
    const buf = await fs.readFile(filepath);
    const bundle = await decode(buf);
    return bundle;
  }

  private async loadFS(): Promise<FSLike> {
    if (this._fs != null) {
      return this._fs;
    }
    const { fs = defaultFS } = this.options;
    if (typeof fs === 'function') {
      this._fs = fs();
    } else {
      this._fs = await fs;
    }
    return this._fs;
  }
}
