import { defineConfig } from '@wvb/cli';
import { loadEnv } from '@wvb-playground/env';

export default defineConfig({
  remote: { endpoint: loadEnv().remote.baseUrl },
  builtin: {
    outDir: './.wvb/bundles',
    include: 'testbed',
  },
});
