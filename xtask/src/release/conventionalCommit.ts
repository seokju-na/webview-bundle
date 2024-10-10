export const CONVENTIONAL_COMMIT_TYPES = [
  'feat',
  'fix',
  'refactor',
  'perf',
  'style',
  'test',
  'docs',
  'build',
  'ops',
  'chore',
] as const;

export type ConventionalCommitType = (typeof CONVENTIONAL_COMMIT_TYPES)[number];

export interface ConventionalCommit {
  type: ConventionalCommitType;
  scopes: string[];
  isBreaking: boolean;
  description: string;
  body?: string;
}

export function parseConventionalCommit(message: string): ConventionalCommit {
  const lines = message.split('\n');
  const [summary, _, ...rest] = lines;
  const body = rest.join('\n');
  const regexp = new RegExp(`^((${CONVENTIONAL_COMMIT_TYPES.join('|')})(\\(.+\\))?(!)?):\\s?([^\\n]+)`);
  const matches = summary?.match(regexp);
  if (matches == null) {
    throw new Error('Message is not conventional');
  }
  const [, , type, scopesRaw, breaking, description] = matches;
  const scopes =
    scopesRaw
      ?.replace('(', '')
      .replace(')', '')
      .split(',')
      .map(x => x.trim()) ?? [];
  const isBreaking = breaking === '!' || message.includes('BREAKING CHANGE:');
  const commit: ConventionalCommit = {
    type: type as ConventionalCommitType,
    scopes,
    isBreaking,
    description: description!,
    body,
  };
  return commit;
}

export function safeParseConventionalCommit(message: string): ConventionalCommit | null {
  try {
    return parseConventionalCommit(message);
  } catch {
    return null;
  }
}
