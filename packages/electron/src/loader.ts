import type { Bundle } from '@webview-bundle/node-binding';
import type { URI } from './uri.js';

export interface Loader {
  getBundleName?(uri: URI): string;
  load(uri: URI): Promise<Bundle>;
}
