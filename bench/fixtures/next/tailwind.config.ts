import type { Config } from 'tailwindcss';
import path from 'node:path';

const config: Config = {
  presets: [require('@vercel/examples-ui/tailwind')],
  content: [
    './pages/**/*.{js,ts,jsx,tsx}',
    './components/**/*.{js,ts,jsx,tsx}',
    path.join(require.resolve('@vercel/examples-ui'), '../**/*.js'),
  ],
};

export default config;
