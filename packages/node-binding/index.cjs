const { Bundle, encode: _encode, decode: _decode, create } = require('./binding.cjs');

async function decode(buf) {
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

async function encode(bundle) {
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

module.exports = {
  Bundle,
  create,
  encode,
  decode,
};
