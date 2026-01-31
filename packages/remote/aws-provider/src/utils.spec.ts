import { describe, expect, it } from 'vitest';
import { toAWSHeaderName } from './utils.js';

describe('toAWSHeaderName', () => {
  it('header name should be capitalized', () => {
    expect(toAWSHeaderName('single')).toEqual('Single');
    expect(toAWSHeaderName('webview-bundle')).toEqual('Webview-Bundle');
    expect(toAWSHeaderName('webview-Bundle-vErsion')).toEqual('Webview-Bundle-Version');
  });
});
