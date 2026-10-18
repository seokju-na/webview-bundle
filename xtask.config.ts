import { defineConfig } from './xtask/config.ts';

export default defineConfig({
  packages: [
    'packages/*',
    'packages/remote/*',
    {
      path: 'packages/create-wvb',
      versionedFilesIgnore: ['template-*/**'],
    },
    {
      path: 'packages/node',
      beforePublishScripts: [{ command: 'yarn', args: ['artifacts'] }],
      artifacts: [{ src: '.', patterns: ['*.node'], dest: 'artifacts' }],
      assets: ['artifacts/*.node'],
    },
    {
      path: 'packages/ffi',
      artifacts: [
        {
          src: '.output',
          patterns: ['apple.zip', 'WebViewBundleFFI.xcframework.zip', 'android.zip'],
          dest: '.output',
        },
      ],
      assets: [
        '.output/android.zip',
        '.output/apple.zip',
        '.output/WebViewBundleFFI.xcframework.zip',
      ],
    },
    {
      path: 'packages/deno',
      artifacts: [
        {
          src: '.output',
          patterns: ['libwvb_deno-*.dylib', 'libwvb_deno-*.so', 'wvb_deno-*.dll', '*.sha256'],
          dest: '.output',
        },
      ],
      assets: [
        '.output/libwvb_deno-*.dylib',
        '.output/libwvb_deno-*.so',
        '.output/wvb_deno-*.dll',
        '.output/*.sha256',
      ],
    },
  ],
});
