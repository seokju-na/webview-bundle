import { ipcMain } from 'electron/main';
import type { IpcErrorPayload } from './errors.js';
import { IpcChannels } from './ipc-channels.js';
import type { Loader } from './loader.js';

interface IpcSuccess<T> {
  success: true;
  data: T;
}

interface IpcError extends IpcErrorPayload {
  success: false;
}

export type IpcResult<T = unknown> = IpcSuccess<T> | IpcError;

export interface RegisterIpcConfig {
  loader: Loader;
  versionFile?: string;
}

export type UnregisterIpc = () => void;

export function registerIpc(config: RegisterIpcConfig): UnregisterIpc {
  const { loader, versionFile: versionFilePath = '__VERSION__' } = config;

  ipcMain.handle(IpcChannels.GetBundleVersion, async (_, params: { name: string }): Promise<IpcResult<string>> => {
    const { name } = params;
    const bundle = await loader.load(name);
    try {
      const versionFile = await bundle.readFileData(versionFilePath);
      const version = versionFile.toString('utf8');
      return { success: true, data: version };
    } catch {
      return {
        success: false,
        code: 'BUNDLE_VERSION_NOT_FOUND',
      };
    }
  });

  return () => {
    ipcMain.removeHandler(IpcChannels.GetBundleVersion);
  };
}
