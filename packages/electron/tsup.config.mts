import glob from 'fast-glob';
import { defineConfig } from 'tsup';

const entry = await glob('src/**/*.ts', {
  onlyFiles: true,
  ignore: ['**/*.spec.ts', '**/*.test.ts'],
});

export default defineConfig({
  entry,
  format: ['cjs', 'esm'],
  splitting: false,
  bundle: false,
  clean: true,
  dts: true,
  platform: 'node',
  target: 'node18',
});
