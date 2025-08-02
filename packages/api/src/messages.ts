import { env } from './env.js';
import { invoke } from './invoke.js';

function command(cmd: string): string {
  if (env.isTauri) {
    return `plugin:webview-bundle|${cmd}`;
  }
  return cmd;
}

export async function getBundleVersion(bundle: string): Promise<string> {
  return invoke<string>(command('get-bundle-version'), { bundle });
}

export async function getBundleMetadata<T = unknown>(bundle: string): Promise<T> {
  return invoke<T>(command('get-bundle-metadata'), { bundle });
}
