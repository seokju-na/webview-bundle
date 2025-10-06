import type { BundleSource, Remote } from '@webview-bundle/electron/binding';
import { type Protocol, registerProtocol } from './protocol.js';
import { type RemoteOptions, remote } from './remote.js';
import { bundleSource, type SourceOptions } from './source.js';

export interface WebviewBundleRemoteConfig extends RemoteOptions {
  endpoint: string;
}

export interface WebviewBundleConfig {
  source?: SourceOptions;
  remote?: WebviewBundleRemoteConfig;
  protocols: Protocol[];
}

export class WebviewBundle {
  private readonly _source: BundleSource;
  private _remote: Remote | null = null;
  private readonly _whenProtocolRegistered: Promise<void>;

  constructor(private readonly config: WebviewBundleConfig) {
    this._source = bundleSource(config.source);
    if (config.remote != null) {
      const { endpoint, ...remoteOptions } = config.remote;
      this._remote = remote(endpoint, remoteOptions);
    }
    this._whenProtocolRegistered = new Promise<void>((resolve, reject) => {
      Promise.all(config.protocols.map(p => registerProtocol(p, this._source)))
        .then(() => resolve())
        .catch(e => reject(e));
    });
  }

  get protocolSchemes(): readonly string[] {
    return this.config.protocols.map(x => x.scheme);
  }

  get source(): BundleSource {
    return this._source;
  }

  get remote(): Remote | null {
    return this._remote;
  }

  setRemote(endpoint: string, options?: RemoteOptions): this {
    this._remote = remote(endpoint, options);
    return this;
  }

  whenProtocolRegistered(): Promise<void> {
    return this._whenProtocolRegistered;
  }
}

export function webviewBundle(config: WebviewBundleConfig): WebviewBundle {
  return new WebviewBundle(config);
}

export const wvb: typeof webviewBundle = webviewBundle;
