import { expect, it } from 'vitest';
import cli from '../binding.cjs.js';

it('can run cli', async () => {
  await cli.run([]);
  await cli.run(['--version']);
});

it('unknown command', async () => {
  await expect(cli.run(['unknown'])).rejects.toThrowError();
});
