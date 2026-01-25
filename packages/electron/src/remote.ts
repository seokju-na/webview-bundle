import { Remote, type RemoteOptions as RemoteBindingOptions } from '@wvb/node';

export interface RemoteOptions extends RemoteBindingOptions {}

export function remote(endpoint: string, options?: RemoteOptions): Remote {
  return new Remote(endpoint, options);
}
