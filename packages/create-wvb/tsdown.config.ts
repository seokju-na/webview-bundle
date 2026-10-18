import { defineConfig } from 'tsdown';

export default defineConfig({
  entry: ['./src/main.ts'],
  format: 'esm',
  platform: 'node',
  target: 'node22',
  minify: true,
  dts: false,
  clean: true,
  fixedExtension: false,
  deps: {
    onlyBundle: false,
  },
});
