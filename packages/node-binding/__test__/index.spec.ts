import { expect, it } from 'vitest';
import { Bundle, create, decode, encode } from '../index.js';

it('parse failure', async () => {
  await expect(decode(Buffer.from([]))).rejects.toThrowError(new Error('empty buffer'));
  await expect(decode(Buffer.from([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]))).rejects.toThrowError(
    'invalid magic number'
  );
});

it('encode empty bundle', async () => {
  const bundle = new Bundle();
  await expect(encode(bundle)).resolves.toEqual(
    Buffer.from([240, 159, 140, 144, 240, 159, 142, 129, 1, 0, 0, 0, 1, 0, 93, 190, 64, 6, 3, 142, 7, 8])
  );
});

it('file not found error', async () => {
  const bundle = new Bundle();
  await expect(bundle.readFileData('index.js')).rejects.toThrowError('file not found');
  await expect(bundle.readFileData('unknown/dir/index.html')).rejects.toThrowError('file not found');
});

it('create bundle', async () => {
  const data = Buffer.from('export const A = 10;', 'utf8');
  const bundle = await create([{ path: 'index.js', data }]);
  await expect(bundle.readFileData('index.js')).resolves.toEqual(data);
});
