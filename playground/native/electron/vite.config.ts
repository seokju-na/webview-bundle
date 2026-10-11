import path from 'node:path';
import { loadEnv } from '@wvb-playground/env';
import { defineConfig } from 'vite';

const env = loadEnv();

export default defineConfig({
  define: {
    'process.env.WVB_PLAYGROUND_REMOTE_BASE_URL': JSON.stringify(env.remote.baseUrl),
  },
  build: {
    lib: {
      entry: path.resolve(import.meta.dirname, 'src/main.ts'),
      formats: ['es'],
      fileName: () => 'main.mjs',
    },
    rolldownOptions: {
      platform: 'node',
      external: ['electron', '@wvb/electron', /^node:/],
    },
  },
});
