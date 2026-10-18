import { defineConfig } from '@wvb/cli';

export default defineConfig({
  builtin: {
    outDir: './.wvb/builtin/bundles',
    target: { type: 'local', workspaces: ['./webview'] },
  },
});
