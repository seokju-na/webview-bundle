import type { Bundle } from '@webview-bundle/node-binding';
import type { URI } from './URI';

export interface Loader {
  getBundleName?(uri: URI): string;
  load(uri: URI): Promise<Bundle>;
}
