import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    emptyOutDir: false,
    rolldownOptions: {
      platform: 'node',
      external: ['electron', /^node:/],
      output: {
        format: 'cjs',
        entryFileNames: 'preload.cjs',
        inlineDynamicImports: true,
      },
    },
  },
});
