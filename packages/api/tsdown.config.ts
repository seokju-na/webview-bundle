import { defineConfig } from 'tsdown';

const config: ReturnType<typeof defineConfig> = defineConfig({
  entry: ['./src/index.ts'],
  format: ['esm', 'cjs'],
  dts: true,
  platform: 'browser',
  target: 'es2020',
});
export { config as default };
