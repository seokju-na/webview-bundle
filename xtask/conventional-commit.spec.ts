import { describe, expect, it } from 'vitest';
import { ConventionalCommit } from './conventional-commit.ts';

describe('ConventionalCommit', () => {
  it('parse', () => {
    const commit = ConventionalCommit.parse(
      '123',
      `feat(core): implement \`my-module\` crate

This commit implements \`my-module\` crate.`
    );
    expect(commit.sha).toBe('123');
    expect(commit.type).toBe('feat');
    expect(commit.scopes).toEqual(['core']);
    expect(commit.isBreaking).toBe(false);
    expect(commit.summary).toEqual('implement `my-module` crate');
    expect(commit.body).toEqual('This commit implements `my-module` crate.');
  });

  it('multiple scopes', () => {
    const commit = ConventionalCommit.parse('123', 'feat(core, cli): hotfix');
    expect(commit.scopes).toEqual(['core', 'cli']);
  });

  it('is breaking', () => {
    let commit = ConventionalCommit.parse('123', 'feat(core)!: implement `my-module` crate');
    expect(commit.isBreaking).toBe(true);
    commit = ConventionalCommit.parse(
      '123',
      `feat(core): implement \`my-module\` crate

BREAKING CHANGE: remove '.addSomething()' method.`
    );
    expect(commit.isBreaking).toBe(true);
  });

  it('parse fail', () => {
    expect(() => ConventionalCommit.parse('123', 'any commit message')).toThrowError();
    expect(() => ConventionalCommit.parse('123', 'unknown: any commit message')).toThrowError();
  });
});
