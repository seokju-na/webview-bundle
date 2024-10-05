import semver from 'semver';

export class Version {
  static parse(raw: string): Version {
    const sem = semver.parse(raw);
    if (sem == null) {
      throw new Error('Invalid semantic version');
    }
    return new Version(sem);
  }

  constructor(protected sem: semver.SemVer) {}

  compare(other: Version): 1 | 0 | -1 {
    return this.sem.compare(other.sem);
  }

  bump(rule: BumpRule): Version {
    const sem = rule.type === 'prerelease' ? this.sem.inc('prerelease', rule.identifier) : this.sem.inc(rule.type);
    return new Version(sem);
  }

  format() {
    return this.sem.format();
  }
}

export type BumpRule =
  | { type: 'major' }
  | { type: 'minor' }
  | { type: 'patch' }
  | { type: 'prerelease'; identifier: string };
