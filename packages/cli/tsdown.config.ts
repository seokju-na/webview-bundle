import { defineConfig, type UserConfig } from 'tsdown';

const config: UserConfig = defineConfig({
  entry: ['./src/main.ts'],
  format: ['esm'],
  platform: 'node',
  target: 'node20',
});

export { config as default };
