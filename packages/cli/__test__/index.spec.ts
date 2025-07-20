import { createRequire } from 'node:module';
import { expect, it } from 'vitest';

const nodeRequire = createRequire(import.meta.url);
const cli = nodeRequire('../binding.cjs');

it('can run cli', async () => {
  await cli.run([]);
  await cli.run(['--version']);
});

it('unknown command', async () => {
  await expect(cli.run(['unknown'])).rejects.toThrowError();
});
