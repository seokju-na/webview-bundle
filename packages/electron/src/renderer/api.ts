import { IpcError } from '../errors.js';
import type { IpcResult } from '../ipc.js';

interface ElectronApi {
  getBundleVersion: (name: string) => Promise<IpcResult<string>>;
}

interface ElectronPreload {
  __WEBVIEW_BUNDLE__?: ElectronApi;
}

async function runIpcInvoke<T = unknown>(invoke: () => Promise<IpcResult<T>>): Promise<T> {
  const result = await invoke();
  if (!result.success) {
    throw new IpcError(result);
  }
  return result.data;
}

export function getBundleVersion(name: string): Promise<string> {
  return runIpcInvoke(() => getPreload().getBundleVersion(name));
}

function getPreload(): ElectronApi {
  const w = window as ElectronPreload & Window;
  if (w.__WEBVIEW_BUNDLE__ == null) {
    throw new Error('Could not load API. Please check if the preload script was installed correctly.');
  }
  return w.__WEBVIEW_BUNDLE__;
}
