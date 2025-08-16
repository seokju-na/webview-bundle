import { version } from '../../package.json' with { type: 'json' };
import type { Loader } from '../loader.js';
import { UpdaterError } from './errors.js';

export interface UpdaterConfig {
  remotesBaseUrl: string;
  loader: Loader;
  userAgent?: string;
  versionFile?: string;
}

const defaultUserAgent = (): string => {
  return `WebViewBundleUpdater/${version}`;
};

export interface UpdateInfo {
  name: string;
  isAvailable: boolean;
  version?: string;
  previousVersion?: string;
}

export class Updater {
  constructor(public readonly config: UpdaterConfig) {}

  async listRemoteBundles(): Promise<string[]> {
    const { userAgent = defaultUserAgent() } = this.config;
    let resp: Response;
    try {
      resp = await fetch(this.parseUrl().toString(), {
        method: 'GET',
        headers: {
          accept: 'application/json',
          'user-agent': userAgent,
        },
      });
    } catch (e) {
      throw new UpdaterError('REMOTES_FETCH_FAILED', undefined, e);
    }
    return this.parseRemoteResponse<string[]>(resp);
  }

  async getUpdateInfo(name: string): Promise<UpdateInfo> {
    const { loader, userAgent = defaultUserAgent(), versionFile = '__VERSION__' } = this.config;
    let resp: Response;
    try {
      resp = await fetch(this.parseUrl(name).toString(), {
        method: 'HEAD',
        headers: {
          'user-agent': userAgent,
        },
      });
    } catch (e) {
      throw new UpdaterError('REMOTES_FETCH_FAILED', undefined, e);
    }
    const version = resp.headers.get('webview-bundle-version');

    const previousVersion = await loader
      .load(name)
      .then(x => x.readFileData(versionFile))
      .then(x => x.toString('utf8'))
      .catch(() => undefined);
  }

  private parseUrl(...paths: string[]): URL {
    const { remotesBaseUrl } = this.config;
    try {
      const url = new URL(remotesBaseUrl);
      if (paths.length > 0) {
        const [, ...segments] = url.pathname.split('/');
        let pathname = [...segments, ...paths].join('/');
        pathname = `/${pathname}`;
        url.pathname = pathname;
      }
      return url;
    } catch (e) {
      throw new UpdaterError('REMOTES_INVALID_URL', undefined, e);
    }
  }

  private async parseRemoteResponse<T>(resp: Response): Promise<T> {
    const ok = 200 <= resp.status && resp.status < 300;
    if (ok) {
      try {
        const parsed: T = await resp.json();
        return parsed;
      } catch (e) {
        throw new UpdaterError('REMOTES_INVALID_RESPONSE', undefined, e);
      }
    }
    throw new UpdaterError('REMOTES_HTTP_ERROR', `Http status is ${resp.status}`);
  }
}
