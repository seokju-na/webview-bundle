import { defineConfig } from 'tsdown';

const config: ReturnType<typeof defineConfig> = defineConfig({
  entry: ['./src/index.ts', './src/preload/index.ts', './src/updater/index.ts'],
  format: ['esm', 'cjs'],
  dts: true,
  platform: 'node',
  target: 'node16',
});
export { config as default };
