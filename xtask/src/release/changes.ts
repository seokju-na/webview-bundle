import type { Commit, Repository } from '@napi-rs/simple-git';
import type { NonEmptyTuple } from 'type-fest';
import * as gitUtils from '../gitUtils';
import { type ConventionalCommit, safeParseConventionalCommit } from './conventionalCommit';
import type { BumpRule, Version } from './versioning';

export interface Change {
  commit: ConventionalCommit;
  author?: {
    name: string;
    email: string;
  };
  timestamp: number;
}

export class Changes {
  static fromGitTag(repo: Repository, tagName: string, scopes?: string[]) {
    const headRef = repo.head();
    const tag = gitUtils.listTags(repo).find(x => x.name === tagName);
    const commits = gitUtils.listCommits(repo, headRef.target()!, tag?.oid);
    const changes = commits.map(commit => getChangeFromCommit(commit, scopes)).filter(x => x != null);
    if (changes.length === 0) {
      return null;
    }
    return new Changes(changes as any as NonEmptyTuple<Change>);
  }

  static formatChange(change: Change): string {
    const { commit } = change;
    const prefix = commit.isBreaking ? '[BREAKING CHANGE] ' : '';
    const head = `${commit.type}${commit.scopes.length > 0 ? `(${commit.scopes.join(',')})` : ''}`;
    return `- ${prefix}${head}: ${commit.description}`;
  }

  constructor(public readonly changes: NonEmptyTuple<Change>) {}

  calculateBumpRule(version: Version): BumpRule {
    const identifier = version.getPrereleaseIdentifier();
    if (identifier != null) {
      return { type: 'prerelease', identifier };
    }
    if (this.changes.some(x => x.commit.isBreaking)) {
      return { type: 'major' };
    }
    if (this.changes.some(x => x.commit.type === 'feat')) {
      return { type: 'minor' };
    }
    return { type: 'patch' };
  }

  format() {
    return this.changes.map(Changes.formatChange).join('\n');
  }
}

function getChangeFromCommit(commit: Commit, scopes?: string[]): Change | null {
  const conventionalCommit = safeParseConventionalCommit(commit.message() ?? '');
  if (conventionalCommit == null) {
    return null;
  }
  if (scopes != null && scopes.length > 0 && !scopes.some(x => conventionalCommit.scopes.includes(x))) {
    return null;
  }
  const name = commit.author().name();
  const email = commit.author().email();
  const change: Change = {
    commit: conventionalCommit,
    author: name != null && email != null ? { name, email } : undefined,
    timestamp: commit.time().getTime(),
  };
  return change;
}
