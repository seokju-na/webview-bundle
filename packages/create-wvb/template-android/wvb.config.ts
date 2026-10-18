import { defineConfig } from '@wvb/cli';

export default defineConfig({
  builtin: {
    outDir: './app/src/main/assets/bundles',
    target: { type: 'local', workspaces: ['./webview'] },
  },
});
