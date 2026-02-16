import { defineConfig, type UserWorkspaceConfig } from 'vitest/config';

const config: UserWorkspaceConfig = defineConfig({
  test: {
    projects: ['packages/*', 'xtask'],
  },
});

export default config;
