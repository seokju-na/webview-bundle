import { Remote, type RemoteOptions as RemoteBindingOptions } from '@webview-bundle/node';

export interface RemoteOptions extends RemoteBindingOptions {}

export function remote(endpoint: string, options?: RemoteOptions): Remote {
  return new Remote(endpoint, options);
}
