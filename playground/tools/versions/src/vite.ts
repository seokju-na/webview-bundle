import type { Plugin } from 'vite';
import { getPickedVersion, versionDistDir, versionRootDir } from './index.ts';

export function versionsPlugin(): Plugin {
  return {
    name: 'wvb:playground:versions',
    config: config => {
      const pickedVersion = getPickedVersion();

      config.root = versionRootDir(pickedVersion);
      config.build ??= {};
      config.build.outDir = versionDistDir(pickedVersion);
      config.build.emptyOutDir = true;
    },
  };
}
