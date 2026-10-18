import { defineConfig } from '@wvb/cli';
import { loadEnv } from '@wvb-playground/env';

const env = loadEnv();

export default defineConfig({
  builtin: { include: 'testbed' },
  remote: {
    baseUrl: env.remote.baseUrl,
  },
});
