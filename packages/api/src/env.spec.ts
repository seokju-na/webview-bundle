import { describe, expect, it } from 'vitest';
import { parseUserAgent } from './env.js';

describe('parseUserAgent', () => {
  describe('valid user agent strings', () => {
    it('should parse complete user agent with all fields', () => {
      const userAgent = 'WebViewBundle/1.0.0 (Electron 22.3.27; win32; arm64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.0.0',
        platform: 'electron',
        platformVersion: '22.3.27',
        os: 'win32',
        arch: 'arm64',
      });
    });

    it('should parse user agent with prerelease version', () => {
      const userAgent = 'WebViewBundle/0.0.0-beta.123 (Electron 22.3.27-alpha.1; win32; arm64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '0.0.0-beta.123',
        platform: 'electron',
        platformVersion: '22.3.27-alpha.1',
        os: 'win32',
        arch: 'arm64',
      });
    });

    it('should parse user agent with build metadata', () => {
      const userAgent = 'WebViewBundle/1.2.3+build.1 (Tauri 1.0.0+exp.sha.5114f85; linux; x64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.2.3+build.1',
        platform: 'tauri',
        platformVersion: '1.0.0+exp.sha.5114f85',
        os: 'linux',
        arch: 'x64',
      });
    });

    it('should parse user agent with complex prerelease and build metadata', () => {
      const userAgent = 'WebViewBundle/1.2.3-beta.1+exp.sha.5114f85 (Electron 25.0.0-nightly.20231201; win32; x64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.2.3-beta.1+exp.sha.5114f85',
        platform: 'electron',
        platformVersion: '25.0.0-nightly.20231201',
        os: 'win32',
        arch: 'x64',
      });
    });

    it('should parse user agent with Tauri platform', () => {
      const userAgent = 'WebViewBundle/2.1.0 (Tauri 1.5.0; darwin; arm64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '2.1.0',
        platform: 'tauri',
        platformVersion: '1.5.0',
        os: 'darwin',
        arch: 'arm64',
      });
    });

    it('should handle case insensitive platform names', () => {
      const userAgent = 'WebViewBundle/1.0.0 (ELECTRON 22.3.27; win32; arm64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.0.0',
        platform: 'electron',
        platformVersion: '22.3.27',
        os: 'win32',
        arch: 'arm64',
      });
    });

    it('should parse user agent with extra text after closing parenthesis', () => {
      const userAgent = 'WebViewBundle/0.0.0 (Electron 22.3.27; win32; arm64) extra text here';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '0.0.0',
        platform: 'electron',
        platformVersion: '22.3.27',
        os: 'win32',
        arch: 'arm64',
      });
    });
  });

  describe('optional fields', () => {
    it('should parse user agent with missing arch field', () => {
      const userAgent = 'WebViewBundle/1.0.0 (Electron 22.3.27; linux)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.0.0',
        platform: 'electron',
        platformVersion: '22.3.27',
        os: 'linux',
        arch: undefined,
      });
    });

    it('should parse user agent with missing os and arch fields', () => {
      const userAgent = 'WebViewBundle/2.0.0 (Tauri 1.0.0)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '2.0.0',
        platform: 'tauri',
        platformVersion: '1.0.0',
        os: undefined,
        arch: undefined,
      });
    });

    it('should parse user agent with empty os field but present arch', () => {
      const userAgent = 'WebViewBundle/1.5.0 (Electron 25.0.0; ; x64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.5.0',
        platform: 'electron',
        platformVersion: '25.0.0',
        os: undefined,
        arch: 'x64',
      });
    });

    it('should parse user agent with os field but empty arch', () => {
      const userAgent = 'WebViewBundle/3.1.0 (Tauri 2.0.0; macos; )';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '3.1.0',
        platform: 'tauri',
        platformVersion: '2.0.0',
        os: 'macos',
        arch: undefined,
      });
    });
  });

  describe('whitespace handling', () => {
    it('should handle extra whitespace around components', () => {
      const userAgent = 'WebViewBundle/1.0.0   (  Electron   22.3.27  ;  win32  ;  arm64  )';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.0.0',
        platform: 'electron',
        platformVersion: '22.3.27',
        os: 'win32',
        arch: 'arm64',
      });
    });

    it('should handle minimal whitespace', () => {
      const userAgent = 'WebViewBundle/1.0.0(Electron 22.3.27;win32;arm64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.0.0',
        platform: 'electron',
        platformVersion: '22.3.27',
        os: 'win32',
        arch: 'arm64',
      });
    });
  });

  describe('version formats', () => {
    it('should parse simple major.minor.patch version', () => {
      const userAgent = 'WebViewBundle/1.2.3 (Electron 22.3.27; linux; x64)';
      const result = parseUserAgent(userAgent);

      expect(result?.version).toBe('1.2.3');
    });

    it('should parse version with four components', () => {
      const userAgent = 'WebViewBundle/1.2.3.4 (Electron 22.3.27.1; linux; x64)';
      const result = parseUserAgent(userAgent);

      expect(result?.version).toBe('1.2.3.4');
      expect(result?.platformVersion).toBe('22.3.27.1');
    });

    it('should parse complex prerelease versions', () => {
      const userAgent = 'WebViewBundle/3.0.0-alpha.beta.1 (Electron 95.0.4638.69-dev.20211015; linux; arm64)';
      const result = parseUserAgent(userAgent);

      expect(result?.version).toBe('3.0.0-alpha.beta.1');
      expect(result?.platformVersion).toBe('95.0.4638.69-dev.20211015');
    });

    it('should parse version with mixed alphanumeric prerelease', () => {
      const userAgent = 'WebViewBundle/1.0.0-x.7.z.92 (Tauri 1.0.0-canary.123.456; darwin; arm64)';
      const result = parseUserAgent(userAgent);

      expect(result?.version).toBe('1.0.0-x.7.z.92');
      expect(result?.platformVersion).toBe('1.0.0-canary.123.456');
    });
  });

  describe('invalid user agent strings', () => {
    it('should return null for empty string', () => {
      const result = parseUserAgent('');
      expect(result).toBeNull();
    });

    it('should return null for string not starting with WebViewBundle', () => {
      const userAgent = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)';
      const result = parseUserAgent(userAgent);
      expect(result).toBeNull();
    });

    it('should return null for malformed WebViewBundle string', () => {
      const userAgent = 'WebViewBundle/1.0.0';
      const result = parseUserAgent(userAgent);
      expect(result).toBeNull();
    });

    it('should return null for missing version', () => {
      const userAgent = 'WebViewBundle/ (Electron 22.3.27; win32; arm64)';
      const result = parseUserAgent(userAgent);
      expect(result).toBeNull();
    });

    it('should return null for missing platform', () => {
      const userAgent = 'WebViewBundle/1.0.0 ( 22.3.27; win32; arm64)';
      const result = parseUserAgent(userAgent);
      expect(result).toBeNull();
    });

    it('should return null for missing platform version', () => {
      const userAgent = 'WebViewBundle/1.0.0 (Electron ; win32; arm64)';
      const result = parseUserAgent(userAgent);
      expect(result).toBeNull();
    });

    it('should return null for unsupported platform', () => {
      const userAgent = 'WebViewBundle/1.0.0 (Chrome 22.3.27; win32; arm64)';
      const result = parseUserAgent(userAgent);
      expect(result).toBeNull();
    });

    it('should return null for missing closing parenthesis', () => {
      const userAgent = 'WebViewBundle/1.0.0 (Electron 22.3.27; win32; arm64';
      const result = parseUserAgent(userAgent);
      expect(result).toBeNull();
    });

    it('should return null for missing opening parenthesis', () => {
      const userAgent = 'WebViewBundle/1.0.0 Electron 22.3.27; win32; arm64)';
      const result = parseUserAgent(userAgent);
      expect(result).toBeNull();
    });

    it('should return null for invalid version', () => {
      const userAgent = 'WebViewBundle/1.0.0 (Electron not_a_version; win32; arm64)';
      const result = parseUserAgent(userAgent);
      expect(result).toBeNull();
    });

    it('should return null for completely invalid format', () => {
      const userAgent = 'invalid user agent string';
      const result = parseUserAgent(userAgent);
      expect(result).toBeNull();
    });
  });

  describe('edge cases', () => {
    it('should handle user agent with special characters in os/arch', () => {
      const userAgent = 'WebViewBundle/1.0.0 (Electron 22.3.27; win32-special_chars; arm64-v8)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.0.0',
        platform: 'electron',
        platformVersion: '22.3.27',
        os: 'win32-special_chars',
        arch: 'arm64-v8',
      });
    });

    it('should handle very long version strings', () => {
      const longVersion = '1.2.3-very.long.prerelease.identifier.with.many.parts+build.metadata.also.very.long';
      const userAgent = `WebViewBundle/${longVersion} (Electron 22.3.27; linux; x64)`;
      const result = parseUserAgent(userAgent);

      expect(result?.version).toBe(longVersion);
    });

    it('should handle case-insensitive WebViewBundle prefix', () => {
      const userAgent = 'webviewbundle/1.0.0 (Electron 22.3.27; win32; arm64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.0.0',
        platform: 'electron',
        platformVersion: '22.3.27',
        os: 'win32',
        arch: 'arm64',
      });
    });

    it('should handle mixed case platform names', () => {
      const userAgent = 'WebViewBundle/1.0.0 (TauRI 1.5.0; darwin; arm64)';
      const result = parseUserAgent(userAgent);

      expect(result?.platform).toBe('tauri');
    });
  });

  describe('real-world examples', () => {
    it('should parse typical Electron app user agent', () => {
      const userAgent = 'WebViewBundle/1.0.0 (Electron 22.3.27; win32; arm64) MyApp/1.0.0';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '1.0.0',
        platform: 'electron',
        platformVersion: '22.3.27',
        os: 'win32',
        arch: 'arm64',
      });
    });

    it('should parse typical Tauri app user agent', () => {
      const userAgent = 'WebViewBundle/2.1.0-rc.1 (Tauri 1.5.0-beta.2; darwin; arm64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '2.1.0-rc.1',
        platform: 'tauri',
        platformVersion: '1.5.0-beta.2',
        os: 'darwin',
        arch: 'arm64',
      });
    });

    it('should parse development build user agent', () => {
      const userAgent = 'WebViewBundle/0.0.0-dev.123+local (Electron 25.0.0-nightly.20231201+sha.abc123; linux; x64)';
      const result = parseUserAgent(userAgent);

      expect(result).toEqual({
        version: '0.0.0-dev.123+local',
        platform: 'electron',
        platformVersion: '25.0.0-nightly.20231201+sha.abc123',
        os: 'linux',
        arch: 'x64',
      });
    });
  });
});
