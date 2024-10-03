import { expect, it } from 'vitest';
import binding from '../index';

it('parse failure', async () => {
  await expect(binding.parse(Buffer.from([]))).rejects.toThrowError(new Error('empty buffer'));
  await expect(binding.parse(Buffer.from([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]))).rejects.toThrowError(
    new Error('header magic mismatch')
  );
});

it('encode empty bundle', async () => {
  const bundle = new binding.Bundle();
  expect(bundle.encode()).toEqual(
    Buffer.from([0xf0, 0x9f, 0x8c, 0x90, 0xf0, 0x9f, 0x8e, 0x81, 0x76, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00])
  );
});
