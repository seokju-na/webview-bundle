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

export type Changes = NonEmptyTuple<Change>;

export function getChangesFromGit(repo: Repository, name: string, version: Version, scopes: string[]): Changes | null {
  const head = repo.head();
  const toTag = gitUtils.listTags(repo).find(x => x.ref === `refs/tags/${name}/v${version.format()}`);
  const commits = gitUtils.listCommits(repo, head.target()!, toTag?.oid);
  const changes = commits.map(commit => getChangeFromCommit(commit, scopes)).filter(x => x != null);
  if (changes.length === 0) {
    return null;
  }
  return changes as any as Changes;
}

function getChangeFromCommit(commit: Commit, scopes: string[]): Change | null {
  const conventionalCommit = safeParseConventionalCommit(commit.message() ?? '');
  if (conventionalCommit == null) {
    return null;
  }
  if (scopes.length > 0 && !scopes.some(x => conventionalCommit.scopes.includes(x))) {
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

export function calcBumpRuleFromChanges(version: Version, changes: Changes): BumpRule {
  const identifier = version.getPrereleaseIdentifier();
  if (identifier != null) {
    return { type: 'prerelease', identifier };
  }
  if (changes.some(x => x.commit.isBreaking)) {
    return { type: 'major' };
  }
  if (changes.some(x => x.commit.type === 'feat')) {
    return { type: 'minor' };
  }
  return { type: 'patch' };
}
