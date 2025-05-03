import { expect, it } from 'vitest';
import binding from '../index';

it('parse failure', async () => {
  await expect(binding.decode(Buffer.from([]))).rejects.toThrowError(new Error('empty buffer'));
  await expect(binding.decode(Buffer.from([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]))).rejects.toThrowError(
    'header magic mismatch'
  );
});

it('encode empty bundle', async () => {
  const bundle = new binding.Bundle();
  await expect(binding.encode(bundle)).resolves.toEqual(
    Buffer.from([0xf0, 0x9f, 0x8c, 0x90, 0xf0, 0x9f, 0x8e, 0x81, 0x76, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00])
  );
});

it('file not found error', async () => {
  const bundle = new binding.Bundle();
  await expect(bundle.readFile('index.js')).rejects.toThrowError('file not found');
  await expect(bundle.readFile('unknown/dir/index.html')).rejects.toThrowError('file not found');
});

it('create bundle', async () => {
  const data = Buffer.from('export const A = 10;', 'utf8');
  const bundle = await binding.create([{ path: 'index.js', data }]);
  await expect(bundle.readFile('index.js')).resolves.toEqual(data);
});
