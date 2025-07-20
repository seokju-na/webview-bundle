import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['./src/index.ts'],
  format: ['esm', 'cjs'],
  splitting: false,
  bundle: true,
  clean: true,
  dts: true,
  platform: 'node',
  target: 'node18',
});
