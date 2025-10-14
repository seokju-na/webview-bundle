import { defineConfig, type UserConfig } from 'tsdown';

const config: UserConfig = defineConfig({
  entry: ['./src/index.ts', './src/preload/index.ts', './src/renderer/index.ts'],
  format: ['esm', 'cjs'],
  platform: 'node',
  target: 'node12',
  dts: true,
  clean: true,
});

export { config as default };
