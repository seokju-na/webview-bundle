import { defineConfig, type UserConfig } from 'tsdown';

const config: UserConfig = defineConfig({
  entry: ['./src/index.ts', './src/origin-request/index.ts', './src/origin-response/index.ts'],
  format: ['esm', 'cjs'],
  platform: 'node',
  target: 'node12',
  dts: true,
  clean: true,
  outExtensions: context => {
    if (context.format === 'cjs') {
      return { js: '.cjs', dts: '.cts' };
    }
    return { js: '.js', dts: '.ts' };
  },
});

export { config as default };
