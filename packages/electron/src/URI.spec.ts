import { describe, expect, it } from 'vitest';
import { URI } from './URI';

describe('URI', () => {
  it('parse URI string and can convert to string', () => {
    const uri = new URI('app://username:password@host:80/path/1/2/3?foo=bar#hash');
    expect(uri.scheme).toEqual('app');
    expect(uri.protocol).toEqual('app:');
    expect(uri.userinfo).toEqual('username:password');
    expect(uri.host).toEqual('host');
    expect(uri.port).toEqual('80');
    expect(uri.path).toEqual('/path/1/2/3');
    expect(uri.query).toEqual('foo=bar');
    expect(uri.fragment).toEqual('hash');
    expect(uri.toString()).toEqual('app://username:password@host:80/path/1/2/3?foo=bar#hash');
  });

  it('parse URI path ends with slash', () => {
    const uri = new URI('app://out.wvb/category/3/');
    expect(uri.path).toEqual('/category/3');
  });

  it('parse URI with empty path', () => {
    const uri = new URI('app://host');
    expect(uri.path).toEqual('/');
  });

  it('parse URI with empty query', () => {
    const uri = new URI('app://host/path?');
    expect(uri.query).toEqual('');
  });

  it('error when URI is invalid', () => {
    const testCases = ['no_uri_spec_string', 'scheme:hostname/path/1/2/3'];
    for (const testCase of testCases) {
      expect(() => new URI(testCase)).toThrowError(TypeError);
    }
  });
});
