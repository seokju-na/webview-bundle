import { defineConfig, type UserConfig } from 'tsdown';

const config: UserConfig = defineConfig({
  entry: ['./src/main.ts'],
  external: ['@webview-bundle/cli/binding'],
  format: ['esm'],
  platform: 'node',
  target: 'node18',
});

export { config as default };
