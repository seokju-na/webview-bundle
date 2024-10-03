const binding = require('./binding');

async function parse(buf) {
  if (buf.length === 0) {
    throw new Error('empty buffer');
  }
  return binding.parse(buf);
}

module.exports.Bundle = binding.Bundle;
module.exports.parse = parse;
