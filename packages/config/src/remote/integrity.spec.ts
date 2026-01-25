import { Buffer } from 'node:buffer';
import { describe, expect, it } from 'vitest';
import { makeIntegrity } from './integrity.js';

describe('makeIntegrity', () => {
  it('sha256 (default)', async () => {
    const data = Buffer.from('hello');
    const integrity = await makeIntegrity({}, data);
    expect(integrity).toEqual('sha256:2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824');
  });

  it('sha384', async () => {
    const data = Buffer.from('hello');
    const integrity = await makeIntegrity({ algorithm: 'sha384' }, data);
    expect(integrity).toEqual(
      'sha384:59e1748777448c69de6b800d7a33bbfb9ff1b463e44354c3553bcdb9c666fa90125a3c79f90397bdf5f6a13de828684f'
    );
  });

  it('sha512', async () => {
    const data = Buffer.from('hello');
    const integrity = await makeIntegrity({ algorithm: 'sha512' }, data);
    expect(integrity).toEqual(
      'sha512:9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043'
    );
  });

  it('produces deterministic output for same input', async () => {
    const data = Buffer.from('same');
    const a = await makeIntegrity({ algorithm: 'sha384' }, data);
    const b = await makeIntegrity({ algorithm: 'sha384' }, data);
    expect(a).toBe(b);
  });
});
