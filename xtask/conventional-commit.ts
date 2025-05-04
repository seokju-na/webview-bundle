import type { Commit } from 'es-git';

const CONVENTIONAL_COMMIT_TYPES = [
  'feat',
  'fix',
  'refactor',
  'perf',
  'test',
  'style',
  'doc',
  'docs',
  'build',
  'ops',
] as const;

export type ConventionalCommitType = (typeof CONVENTIONAL_COMMIT_TYPES)[number];

export function isConventionalCommitType(x: unknown): x is ConventionalCommitType {
  return CONVENTIONAL_COMMIT_TYPES.some(type => type === x);
}

export class ConventionalCommit {
  public readonly sha: string;
  public readonly type: ConventionalCommitType;
  public readonly scopes: string[];
  public readonly isBreaking: boolean;
  public readonly summary: string;
  public readonly body?: string;

  static parse(sha: string, message: string): ConventionalCommit {
    const lines = message.split('\n');
    const [title = '', _, ...bodyLines] = lines;
    const m = /^(?<type>\w+)(?:\((?<scopes>.+)\))?(?<breaking>!)?: (?<summary>.+)$/.exec(title);
    const { type, scopes, breaking, summary } = m?.groups ?? {};
    if (!isConventionalCommitType(type)) {
      throw new Error(`invalid commit type: ${type}`);
    }
    if (summary == null) {
      throw new Error(`invalid commit summary: ${summary}`);
    }
    const body = bodyLines.length === 0 ? undefined : bodyLines.join('\n');
    return new ConventionalCommit(
      sha,
      type,
      scopes?.split(',').map(x => x.trim()) ?? [],
      breaking != null || body?.includes('BREAKING CHANGE:') === true,
      summary,
      body
    );
  }

  static parseCommit(commit: Commit): ConventionalCommit {
    const sha = commit.id();
    const message = commit.message();
    return ConventionalCommit.parse(sha, message);
  }

  private constructor(
    sha: string,
    type: ConventionalCommitType,
    scopes: string[],
    isBreaking: boolean,
    summary: string,
    body?: string
  ) {
    this.sha = sha;
    this.type = type;
    this.scopes = scopes;
    this.isBreaking = isBreaking;
    this.summary = summary;
    this.body = body;
  }
}
