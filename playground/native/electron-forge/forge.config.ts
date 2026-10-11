import type { ForgeConfig } from '@electron-forge/shared-types';
import { WebviewBundlePlugin } from '@wvb/electron-forge';

const config: ForgeConfig = {
  packagerConfig: {
    name: 'WvbElectronForge',
    appBundleId: 'dev.wvb.playground.forge',
    // Yarn workspaces link @wvb/electron outside this app directory.
    derefSymlinks: true,
    asar: { unpack: '**/*.node' },
    ignore: [/^\/(src|out|\.wvb)(\/|$)/, /\.config\.(ts|cjs)$/],
  },
  makers: [{ name: '@electron-forge/maker-dmg', config: { format: 'UDZO' } }],
  plugins: [new WebviewBundlePlugin({ root: import.meta.dirname })],
};

export default config;
