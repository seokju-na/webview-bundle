const binding = require('./binding');

async function decode(buf) {
  if (buf.length === 0) {
    throw new Error('empty buffer');
  }
  return new Promise((resolve, reject) => {
    binding.decode(buf, (err, result) => {
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
    binding.encode(bundle, (err, result) => {
      if (err != null) {
        reject(err);
      } else {
        resolve(result);
      }
    });
  });
}

module.exports.Bundle = binding.Bundle;
module.exports.decode = decode;
module.exports.encode = encode;
module.exports.create = binding.create;
