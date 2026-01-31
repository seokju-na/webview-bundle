import { defineConfig, type UserConfig } from 'tsdown';

const config: UserConfig = defineConfig({
  entry: ['./src/index.ts'],
  format: ['esm', 'cjs'],
  platform: 'neutral',
  target: ['node12', 'es2018'],
  dts: true,
});

export { config as default };
