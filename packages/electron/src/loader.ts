import type { Bundle } from 'packages/node-binding-t';

export interface Loader {
  load(name: string): Promise<Bundle>;
}
