import { defineConfig } from '@wvb/cli';

export default defineConfig({
  builtin: {
    outDir: './bundles',
    target: { type: 'local', workspaces: ['./webview'] },
  },
});
