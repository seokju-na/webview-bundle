import { VitePlugin } from '@electron-forge/plugin-vite';
import type { ForgeConfig } from '@electron-forge/shared-types';
import { WebviewBundlePlugin } from '@wvb/electron-forge';

const config: ForgeConfig = {
  packagerConfig: {
    asar: { unpack: '**/*.node' },
    // Keep runtime dependencies because @wvb/electron is external to the Vite build.
    ignore: file =>
      file !== '' &&
      file !== '/package.json' &&
      !file.startsWith('/.vite') &&
      !file.startsWith('/node_modules'),
  },
  makers: [{ name: '@electron-forge/maker-zip', config: {} }],
  plugins: [
    new VitePlugin({
      build: [
        { entry: 'src/main.ts', config: 'vite.config.ts', target: 'main' },
        { entry: 'src/preload.ts', config: 'vite.preload.config.ts', target: 'preload' },
      ],
      // The renderer is served by Webview Bundle.
      renderer: [],
    }),
    new WebviewBundlePlugin({ root: import.meta.dirname }),
  ],
};

export default config;
