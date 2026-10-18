import { withWebviewBundle } from '@wvb/electron-builder';

export default withWebviewBundle({
  appId: 'com.example.wvbapp',
  productName: 'WvbApp',
  directories: { output: 'out' },
  files: ['src/**/*'],
  asar: true,
  asarUnpack: ['**/*.node'],
  npmRebuild: false,
}, { root: import.meta.dirname });
