import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);
const { Bundle: _Bundle, encode: _encode, decode: _decode, create: _create } = require('./binding.cjs');

export const Bundle = _Bundle;
export const create = _create;

export async function decode(buf) {
  if (buf.length === 0) {
    throw new Error('empty buffer');
  }
  return new Promise((resolve, reject) => {
    _decode(buf, (err, result) => {
      if (err != null) {
        reject(err);
      } else {
        resolve(result);
      }
    });
  });
}

export async function encode(bundle) {
  return new Promise((resolve, reject) => {
    _encode(bundle, (err, result) => {
      if (err != null) {
        reject(err);
      } else {
        resolve(result);
      }
    });
  });
}
