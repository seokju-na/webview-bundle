import { defineConfig } from 'vite';

// https://vitejs.dev/config
export default defineConfig({
  build: {
    rollupOptions: {
      external: ['@webview-bundle/electron', '@webview-bundle/node'],
    },
  },
});
