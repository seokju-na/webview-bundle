import { defineProject, type UserWorkspaceConfig } from 'vitest/config';

const config: UserWorkspaceConfig = defineProject({
  test: {
    clearMocks: true,
    environment: 'happy-dom',
  },
});
export { config as default };
