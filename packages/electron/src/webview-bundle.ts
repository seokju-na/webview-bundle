import { type Protocol, registerProtocol } from './protocol.js';

export interface WebviewBundleConfig {
  protocols: Protocol[];
}

export function webviewBundle(config: WebviewBundleConfig): void {
  const { protocols } = config;
  Promise.all(protocols.map(p => registerProtocol(p)));
}

export const wvb: typeof webviewBundle = webviewBundle;
