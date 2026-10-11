import { withWebviewBundle } from '@wvb/electron-builder';
import electronPackage from 'electron/package.json' with { type: 'json' };

const config = withWebviewBundle(
  {
    appId: 'dev.wvb.playground.builder',
    productName: 'WvbElectronBuilder',
    electronVersion: electronPackage.version,
    directories: { output: 'out' },
    files: ['dist/**/*'],
    asar: true,
    asarUnpack: ['**/*.node'],
    npmRebuild: false,
    mac: { target: 'dmg', identity: null },
  },
  { root: import.meta.dirname }
);

export default config;
