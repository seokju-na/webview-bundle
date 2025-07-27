import { defineConfig } from 'tsdown';

const config: ReturnType<typeof defineConfig> = defineConfig({
  entry: ['./src/index.ts', './src/meta/index.ts'],
  format: ['esm'],
  platform: 'node',
  target: 'node20',
});

export { config as default };
