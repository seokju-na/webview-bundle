import react from '@vitejs/plugin-react';
import { versionsPlugin } from '@wvb-playground/versions/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [versionsPlugin(), react()],
});
