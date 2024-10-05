import { describe, expect, it } from 'vitest';
import { parseConventionalCommit } from './conventionalCommit';

describe('parseConventionalCommit', () => {
  it('parse successfully', () => {
    const message = `feat(core): implement \`my_module\` crate

This commit is implements \`my_module\` crate.`;
    const conventionalCommit = parseConventionalCommit(message);
    expect(conventionalCommit.type).toEqual('feat');
    expect(conventionalCommit.scopes).toEqual(['core']);
    expect(conventionalCommit.description).toEqual('implement `my_module` crate');
    expect(conventionalCommit.isBreaking).toBe(false);
    expect(conventionalCommit.body).toBe('This commit is implements `my_module` crate.');
  });

  it('multiple scopes', () => {
    const message = 'fix(core, cli): hotfix';
    const conventionalCommit = parseConventionalCommit(message);
    expect(conventionalCommit.scopes).toEqual(['core', 'cli']);
  });

  it('breaking from "!"', () => {
    const message = 'feat!: remove deprecated methods';
    const conventionalCommit = parseConventionalCommit(message);
    expect(conventionalCommit.isBreaking).toBe(true);
  });

  it('breaking from "BREAKING CHANGE:"', () => {
    const message = `feat: remove deprecated methods

BREAKING CHANGE: remove '.addSomething()' method
`;
    const conventionalCommit = parseConventionalCommit(message);
    expect(conventionalCommit.isBreaking).toBe(true);
  });

  it('throw error if message is not conventional commit', () => {
    expect(() => parseConventionalCommit('any commit message')).toThrowError();
    expect(() => parseConventionalCommit('unknown: hello world')).toThrowError();
  });
});
