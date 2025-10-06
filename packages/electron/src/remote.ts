import {
  Remote,
  type RemoteOptions as RemoteBindingOptions,
  type RemoteOnDownloadData,
} from '@webview-bundle/electron/binding';

export interface RemoteOptions extends RemoteBindingOptions {
  onDownload?: (data: RemoteOnDownloadData) => void;
}

export function remote(endpoint: string, options: RemoteOptions = {}): Remote {
  const { onDownload, ...restOptions } = options;
  return new Remote(endpoint, restOptions, onDownload);
}
