import type { Bundle } from '@webview-bundle/node-binding';

export interface Loader {
  load(name: string): Promise<Bundle>;
}
