import type { ForgeConfig } from '@electron-forge/shared-types';
import { WebviewBundlePlugin } from '@wvb/electron-forge';

const config: ForgeConfig = {
  packagerConfig: {
    asar: { unpack: '**/*.node' },
    // Keep runtime dependencies because @wvb/electron is external to the main build.
    ignore: file =>
      file !== '' &&
      file !== '/package.json' &&
      !file.startsWith('/dist') &&
      !file.startsWith('/node_modules'),
  },
  makers: [{ name: '@electron-forge/maker-zip', config: {} }],
  plugins: [new WebviewBundlePlugin()],
};

export default config;
