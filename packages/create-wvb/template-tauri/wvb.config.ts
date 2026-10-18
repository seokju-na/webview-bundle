import { defineConfig } from '@wvb/cli';

export default defineConfig({
  builtin: {
    outDir: './.wvb/bundles',
    target: { type: 'local', workspaces: ['./webview'] },
  },
});
