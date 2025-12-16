import { defineConfig, type UserConfig } from 'tsdown';

const config: UserConfig = defineConfig({
  entry: ['./src/index.ts'],
  format: ['esm', 'cjs'],
  platform: 'node',
  target: 'node12',
  dts: true,
  outExtensions: context => {
    if (context.format === 'cjs') {
      return { js: '.cjs', dts: '.cts' };
    }
    return { js: '.js', dts: '.ts' };
  },
});

export { config as default };
