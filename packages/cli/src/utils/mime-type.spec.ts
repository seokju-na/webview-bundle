import { describe, expect, it } from 'vitest';
import { parseMimeType, parseMimeTypeFromUri } from './mime-type.js';

describe('parseMimeTypeFromUri', () => {
  it('parse specific types based on known extensions', () => {
    expect(parseMimeTypeFromUri('style.css')).toBe('text/css');
    expect(parseMimeTypeFromUri('data.csv')).toBe('text/csv');
    expect(parseMimeTypeFromUri('page.html')).toBe('text/html');
    expect(parseMimeTypeFromUri('favicon.ico')).toBe('image/vnd.microsoft.icon');
    expect(parseMimeTypeFromUri('app.js')).toBe('text/javascript');
    expect(parseMimeTypeFromUri('module.mjs')).toBe('text/javascript');
    expect(parseMimeTypeFromUri('data.json')).toBe('application/json');
    expect(parseMimeTypeFromUri('ld.jsonld')).toBe('application/ld+json');
    expect(parseMimeTypeFromUri('video.mp4')).toBe('video/mp4');
    expect(parseMimeTypeFromUri('doc.rtf')).toBe('application/rtf');
    expect(parseMimeTypeFromUri('vector.svg')).toBe('image/svg+xml');
    expect(parseMimeTypeFromUri('readme.txt')).toBe('text/plain');
    expect(parseMimeTypeFromUri('raw.bin')).toBe('application/octet-stream');
    // also mapped to css
    expect(parseMimeTypeFromUri('style.less')).toBe('text/css');
    expect(parseMimeTypeFromUri('style.sass')).toBe('text/css');
    expect(parseMimeTypeFromUri('style.styl')).toBe('text/css');
  });

  it('parse application/octet-stream when no suffix exists', () => {
    expect(parseMimeTypeFromUri('filename-without-dot')).toBe('application/octet-stream');
  });

  it('parse undefined for unknown extensions', () => {
    expect(parseMimeTypeFromUri('archive.unknownext')).toBeUndefined();
  });
});

describe('parseMimeType', () => {
  const PNG = new Uint8Array([
    // PNG signature
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a,
    // IHDR chunk length (13)
    0x00, 0x00, 0x00, 0x0d,
    // IHDR
    0x49, 0x48, 0x44, 0x52,
    // width=1, height=1
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
    // bit depth=8, color type=RGBA(6), compression=0, filter=0, interlace=0
    0x08, 0x06, 0x00, 0x00, 0x00,
    // CRC for IHDR (correct for the above bytes): 0x1f 0x15 0xc4 0x89
    0x1f, 0x15, 0xc4, 0x89,
  ]);
  const MP4 = new Uint8Array([
    // ftypisom (common for MP4). file-type checks within first bytes.
    0x00,
    0x00,
    0x00,
    0x18, // box size
    0x66,
    0x74,
    0x79,
    0x70, // 'ftyp'
    0x69,
    0x73,
    0x6f,
    0x6d, // 'isom'
    0x00,
    0x00,
    0x02,
    0x00,
    0x69,
    0x73,
    0x6f,
    0x6d, // 'isom'
    0x69,
    0x73,
    0x6f,
    0x32, // 'iso2'
  ]);
  const ICO = new Uint8Array([
    // ICO header: reserved=0(2 bytes), type=1(2 bytes), count>=1(2 bytes)
    0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
  ]);
  const JSON_SAMPLE = new TextEncoder().encode('{"a":1}');
  const TXT_SAMPLE = new TextEncoder().encode('hello');

  it('detects PNG from header regardless of uri', async () => {
    const mime = await parseMimeType(PNG, 'image');
    expect(mime).toBe('image/png');
  });

  it('detects MP4 from header', async () => {
    const mime = await parseMimeType(MP4, 'movie');
    expect(mime).toBe('video/mp4');
  });

  it('detects ICO from header', async () => {
    const mime = await parseMimeType(ICO, 'favicon');
    expect(mime).toBe('image/x-icon');
  });

  it('falls back to URI for SVG (skips header detection by design)', async () => {
    const mime = await parseMimeType(new Uint8Array([0x3c, 0x73, 0x76, 0x67]), 'icon.svg');
    expect(mime).toBe('image/svg+xml');
  });

  it('falls back to URI mapping when header is not indicative (JSON with .json)', async () => {
    const mime = await parseMimeType(JSON_SAMPLE, 'data.json');
    // file-type may or may not recognize pure JSON; fallback ensures correct mapping
    expect(mime).toBe('application/json');
  });

  it('falls back to URI mapping for txt', async () => {
    const mime = await parseMimeType(TXT_SAMPLE, 'readme.txt');
    expect(mime).toBe('text/plain');
  });

  it('unknown extension with unrecognized header defaults to text/html', async () => {
    const random = new Uint8Array([0xde, 0xad, 0xbe, 0xef]);
    const mime = await parseMimeType(random, 'file.unknownext');
    expect(mime).toBe('text/html');
  });

  it('no extension and unrecognized header falls back to application/octet-stream (via URI)', async () => {
    const random = new Uint8Array([0xaa, 0xbb, 0xcc, 0xdd]);
    const mime = await parseMimeType(random, 'noext');
    expect(mime).toBe('application/octet-stream');
  });
});
