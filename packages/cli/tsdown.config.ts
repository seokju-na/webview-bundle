import { defineConfig } from 'tsdown';

const config: ReturnType<typeof defineConfig> = defineConfig({
  entry: ['./src/main.ts'],
  format: ['esm'],
  platform: 'node',
  target: 'node20',
});

export { config as default };
