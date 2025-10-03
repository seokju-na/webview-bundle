import type { Commit, Repository } from 'es-git';
import { isNotNil } from 'es-toolkit';
import { ConventionalCommit } from './conventional-commit.ts';
import type { BumpRule } from './version.ts';
import type { VersionedGitTag } from './versioned-git-tag.ts';

export class Change {
  public readonly commit: ConventionalCommit;

  static tryFromCommit(commit: Commit): Change {
    return new Change(ConventionalCommit.parseCommit(commit));
  }

  constructor(commit: ConventionalCommit) {
    this.commit = commit;
  }

  toString() {
    const { type, scopes, isBreaking, summary } = this.commit;
    const prefix = isBreaking ? '[BREAKING CHANGE] ' : '';
    const scopesStr = scopes.length > 0 ? `(${scopes.join(',')})` : '';
    return `${prefix}${type}${scopesStr}: ${summary}`;
  }
}

export class Changes {
  public readonly changes: ReadonlyArray<Change>;

  static tryFromGitTag(repo: Repository, prevTag: VersionedGitTag, scopes: readonly string[]): Changes {
    const head = repo.head().target();
    if (head == null) {
      throw new Error('cannot find git `HEAD` target');
    }
    const tag = prevTag.findTag(repo);
    const revwalk = repo.revwalk();
    revwalk.push(head);
    if (tag != null) {
      revwalk.hide(tag.id());
    }
    const changes = [...revwalk]
      .map(oid => repo.findCommit(oid))
      .filter(isNotNil)
      .map(commit => {
        try {
          return Change.tryFromCommit(commit);
        } catch {
          return null;
        }
      })
      .filter(isNotNil)
      .filter(change => {
        if (change.commit.scopes.length === 0) {
          return false;
        }
        return change.commit.scopes.some(x => scopes.includes(x));
      });
    return new Changes(changes);
  }

  constructor(changes: Change[]) {
    this.changes = changes;
  }

  getBumpRule(): BumpRule | null {
    if (this.changes.length === 0) {
      return null;
    }
    if (this.changes.some(x => x.commit.isBreaking)) {
      return { type: 'major' };
    }
    if (this.changes.some(x => x.commit.type === 'feat')) {
      return { type: 'minor' };
    }
    return { type: 'patch' };
  }
}
