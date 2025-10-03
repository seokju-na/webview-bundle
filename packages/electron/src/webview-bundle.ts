import { type Protocol, registerProtocol } from './protocol.js';
import { initSource, type SourceOptions } from './source.js';

export interface WebviewBundleConfig {
  source?: SourceOptions;
  protocols: Protocol[];
}

export function webviewBundle(config: WebviewBundleConfig): void {
  const { source, protocols } = config;
  initSource(source);
  Promise.all(protocols.map(p => registerProtocol(p)));
}

export const wvb: typeof webviewBundle = webviewBundle;
