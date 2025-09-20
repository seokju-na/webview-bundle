import { describe, expect, it } from 'vitest';
import { formatByteLength } from './format.js';

describe('formatByteLength', () => {
  it('returns "0 B" for zero', () => {
    expect(formatByteLength(0)).toBe('0 B');
  });

  it('formats Bytes (B) without decimals', () => {
    expect(formatByteLength(1)).toBe('1 B');
    expect(formatByteLength(512)).toBe('512 B');
    expect(formatByteLength(1023)).toBe('1023 B');
  });

  it('formats Kilobytes (KB) with up to 2 decimals', () => {
    expect(formatByteLength(1024)).toBe('1 KB');
    expect(formatByteLength(1536)).toBe('1.5 KB'); // 1.5 KB

    // About 10.35 KB
    const bytesFor1035 = Math.round(10.35 * 1024);
    expect(formatByteLength(bytesFor1035)).toBe('10.35 KB');

    // Rounded to 10.34 KB
    const bytesFor10345 = Math.round(10.345 * 1024);
    expect(formatByteLength(bytesFor10345)).toBe('10.34 KB');
  });

  it('formats larger units (MB/GB/TB)', () => {
    expect(formatByteLength(1024 ** 2)).toBe('1 MB');
    expect(formatByteLength(1024 ** 3)).toBe('1 GB');
    expect(formatByteLength(1024 ** 4)).toBe('1 TB');
  });

  it('handles a typical large number', () => {
    // 123,456,789 bytes ~= 117.74 MB
    expect(formatByteLength(123_456_789)).toBe('117.74 MB');
  });
});
