import { defineConfig, type UserConfig } from 'tsdown';

const config: UserConfig = defineConfig({
  entry: ['./src/main.ts', './src/config.ts'],
  format: ['esm'],
  platform: 'node',
  target: 'node18',
  outputOptions: {
    entryFileNames: '[name].js',
    chunkFileNames: 'chunks/[name].js',
    exports: 'named',
  },
});

export { config as default };
