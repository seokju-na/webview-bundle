const { withWebviewBundle } = require('@wvb/electron-builder');

module.exports = withWebviewBundle({
  appId: 'com.example.wvbapp',
  productName: 'WvbApp',
  directories: { output: 'out' },
  files: ['dist/**/*'],
  asar: true,
  asarUnpack: ['**/*.node'],
  npmRebuild: false,
});
