import { defineConfig, type UserConfig } from 'tsdown';

const config: UserConfig = defineConfig({
  entry: ['./src/index.ts'],
  external: ['@webview-bundle/electron/binding'],
  format: ['esm', 'cjs'],
  platform: 'node',
  target: 'node12',
  dts: true,
});

export { config as default };
