import { Buffer } from 'node:buffer';
import { describe, expect, it } from 'vitest';
import { signSignature } from './signature.js';

describe('signSignature', () => {
  it('ecdsa secp256r1 (sha256)', async () => {
    const keyPair = await crypto.subtle.generateKey({ name: 'ECDSA', namedCurve: 'P-256' }, true, [
      'sign',
      'verify',
    ]);
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'ecdsa',
        curve: 'p256',
        hash: 'sha256',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'ECDSA', hash: 'SHA-256' },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('ecdsa secp256r1 (sha384)', async () => {
    const keyPair = await crypto.subtle.generateKey({ name: 'ECDSA', namedCurve: 'P-256' }, true, [
      'sign',
      'verify',
    ]);
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'ecdsa',
        curve: 'p256',
        hash: 'sha384',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'ECDSA', hash: 'SHA-384' },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('ecdsa secp256r1 (sha512)', async () => {
    const keyPair = await crypto.subtle.generateKey({ name: 'ECDSA', namedCurve: 'P-256' }, true, [
      'sign',
      'verify',
    ]);
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'ecdsa',
        curve: 'p256',
        hash: 'sha512',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'ECDSA', hash: 'SHA-512' },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('ecdsa secp384r1 (sha256)', async () => {
    const keyPair = await crypto.subtle.generateKey({ name: 'ECDSA', namedCurve: 'P-384' }, true, [
      'sign',
      'verify',
    ]);
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'ecdsa',
        curve: 'p384',
        hash: 'sha256',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'ECDSA', hash: 'SHA-256' },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('ecdsa secp384r1 (sha384)', async () => {
    const keyPair = await crypto.subtle.generateKey({ name: 'ECDSA', namedCurve: 'P-384' }, true, [
      'sign',
      'verify',
    ]);
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'ecdsa',
        curve: 'p384',
        hash: 'sha384',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'ECDSA', hash: 'SHA-384' },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('ecdsa secp384r1 (sha512)', async () => {
    const keyPair = await crypto.subtle.generateKey({ name: 'ECDSA', namedCurve: 'P-384' }, true, [
      'sign',
      'verify',
    ]);
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'ecdsa',
        curve: 'p384',
        hash: 'sha512',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'ECDSA', hash: 'SHA-512' },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('ed25519', async () => {
    const keyPair = await crypto.subtle.generateKey(
      {
        name: 'Ed25519',
      },
      true,
      ['sign', 'verify']
    );
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'ed25519',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'Ed25519' },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('rsa-pkcs1-v1_5 (sha256)', async () => {
    const keyPair = await crypto.subtle.generateKey(
      {
        name: 'RSASSA-PKCS1-v1_5',
        modulusLength: 2048,
        publicExponent: new Uint8Array([1, 0, 1]),
        hash: { name: 'SHA-256' },
      },
      true,
      ['sign', 'verify']
    );
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'rsa-pkcs1-v1.5',
        hash: 'sha256',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'RSASSA-PKCS1-v1_5' },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('rsa-pkcs1-v1_5 (sha384)', async () => {
    const keyPair = await crypto.subtle.generateKey(
      {
        name: 'RSASSA-PKCS1-v1_5',
        modulusLength: 2048,
        publicExponent: new Uint8Array([1, 0, 1]),
        hash: { name: 'SHA-384' },
      },
      true,
      ['sign', 'verify']
    );
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'rsa-pkcs1-v1.5',
        hash: 'sha384',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'RSASSA-PKCS1-v1_5' },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('rsa-pkcs1-v1_5 (sha512)', async () => {
    const keyPair = await crypto.subtle.generateKey(
      {
        name: 'RSASSA-PKCS1-v1_5',
        modulusLength: 2048,
        publicExponent: new Uint8Array([1, 0, 1]),
        hash: { name: 'SHA-512' },
      },
      true,
      ['sign', 'verify']
    );
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'rsa-pkcs1-v1.5',
        hash: 'sha512',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'RSASSA-PKCS1-v1_5' },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('rsa-pss (sha256)', async () => {
    const keyPair = await crypto.subtle.generateKey(
      {
        name: 'RSA-PSS',
        modulusLength: 2048,
        publicExponent: new Uint8Array([1, 0, 1]),
        hash: { name: 'SHA-256' },
      },
      true,
      ['sign', 'verify']
    );
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'rsa-pss',
        hash: 'sha256',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'RSA-PSS', saltLength: 32 },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('rsa-pss (sha384)', async () => {
    const keyPair = await crypto.subtle.generateKey(
      {
        name: 'RSA-PSS',
        modulusLength: 2048,
        publicExponent: new Uint8Array([1, 0, 1]),
        hash: { name: 'SHA-384' },
      },
      true,
      ['sign', 'verify']
    );
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'rsa-pss',
        hash: 'sha384',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'RSA-PSS', saltLength: 48 },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });

  it('rsa-pss (sha512)', async () => {
    const keyPair = await crypto.subtle.generateKey(
      {
        name: 'RSA-PSS',
        modulusLength: 2048,
        publicExponent: new Uint8Array([1, 0, 1]),
        hash: { name: 'SHA-512' },
      },
      true,
      ['sign', 'verify']
    );
    const pkcs8 = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    const message = Buffer.from('test message');
    const signature = await signSignature(
      {
        algorithm: 'rsa-pss',
        hash: 'sha512',
        key: {
          format: 'pkcs8',
          data: Buffer.from(pkcs8),
        },
      },
      message
    );
    const verified = await crypto.subtle.verify(
      { name: 'RSA-PSS', saltLength: 64 },
      keyPair.publicKey,
      new Uint8Array(Buffer.from(signature, 'base64')),
      new Uint8Array(message)
    );
    expect(verified).toBe(true);
  });
});
