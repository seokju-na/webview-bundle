import { afterEach, describe, expect, it, vi } from 'vitest';
import { Changelog } from './changelog.ts';
import { Change, Changes } from './changes.ts';
import { Package } from './package.ts';
import { Version } from './version.ts';
import { VersionedFile } from './versioned-file.ts';

function makePackage(): Package {
  const file = VersionedFile.parse(
    'package.json',
    'packages/create-wvb/package.json',
    JSON.stringify({ name: 'create-wvb', version: '0.1.0' })
  )!;
  return new Package('create-wvb', 'packages/create-wvb', [file], {});
}

afterEach(() => vi.restoreAllMocks());

describe('changelog release notes', () => {
  it('extracts only the exact package and version, excluding other sections', () => {
    const changelog = new Changelog(
      'CHANGELOG.md',
      [
        '# Changelog',
        '## create-wvb v0.1.0-next.1',
        '- prerelease',
        '## create-wvb v0.1.0',
        '',
        'Package link',
        '',
        '### Added',
        '- create cli',
        '',
        '## create-webview-bundle v0.1.0',
        '- alias',
      ].join('\n')
    );
    expect(changelog.extractChanges(makePackage())).toBe('Package link\n\n### Added\n- create cli');
  });

  it('does not match a prerelease when the stable version is missing', () => {
    const changelog = new Changelog('CHANGELOG.md', '## create-wvb v0.1.0-next.1\n- prerelease');
    expect(changelog.extractChanges(makePackage())).toBeNull();
  });

  it('appends a stable section without modifying the prerelease section', () => {
    const pkg = makePackage();
    pkg.setVersion(Version.parse('0.2.0'));
    const changelog = new Changelog(
      'CHANGELOG.md',
      '# Changelog\n\n## create-wvb v0.2.0-next.1\n\n- prerelease\n'
    );
    changelog.appendChanges(pkg, new Changes([new Change('stable')]));
    expect(changelog.extractChanges(pkg)).toContain('- stable');
    expect(changelog.extractChanges(pkg)).not.toContain('- prerelease');
  });

  it('uses the root changelog when the package changelog is missing', async () => {
    vi.spyOn(Changelog, 'load').mockImplementation(async filepath => {
      if (filepath !== 'CHANGELOG.md') {
        throw Object.assign(new Error('missing'), { code: 'ENOENT' });
      }
      return new Changelog(filepath, '# Changelog\n\n## create-wvb v0.1.0\n\n- create cli\n');
    });
    expect(await Changelog.releaseNotes(makePackage())).toBe('- create cli');
  });

  it('falls back when the package changelog lacks the requested version', async () => {
    vi.spyOn(Changelog, 'load').mockImplementation(
      async filepath =>
        new Changelog(
          filepath,
          filepath === 'CHANGELOG.md' ? '## create-wvb v0.1.0\n\n- root notes' : '# Changelog\n'
        )
    );
    expect(await Changelog.releaseNotes(makePackage())).toBe('- root notes');
  });

  it('prefers package notes and propagates read errors', async () => {
    const load = vi
      .spyOn(Changelog, 'load')
      .mockResolvedValue(new Changelog('local', '## create-wvb v0.1.0\n\n- local notes'));
    expect(await Changelog.releaseNotes(makePackage())).toBe('- local notes');
    expect(load).toHaveBeenCalledTimes(1);
    load.mockRejectedValue(Object.assign(new Error('permission denied'), { code: 'EACCES' }));
    await expect(Changelog.releaseNotes(makePackage())).rejects.toThrow('permission denied');
  });

  it('rejects missing notes instead of creating an empty release', async () => {
    vi.spyOn(Changelog, 'load').mockResolvedValue(new Changelog('empty', '# Changelog\n'));
    await expect(Changelog.releaseNotes(makePackage())).rejects.toThrow('Missing release notes');
  });
});
