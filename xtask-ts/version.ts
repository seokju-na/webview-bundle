import { type SemVer, parse } from 'semver';

export type BumpRule =
  | { type: 'major' }
  | { type: 'minor' }
  | { type: 'patch' }
  | { type: 'prerelease'; id: string; num: number };

export class Version {
  private ver: SemVer;

  static parse(raw: string): Version {
    const ver = parse(raw);
    if (ver == null) {
      throw new Error(`invalid version: ${raw}`);
    }
    return new Version(ver);
  }

  constructor(ver: SemVer) {
    this.ver = ver;
  }

  get prerelease(): { id: string; num: number } | null {
    const [id, num] = this.ver.prerelease;
    if (typeof id === 'string' && typeof num === 'number') {
      return { id, num };
    }
    return null;
  }

  equals(other: Version): boolean {
    return this.ver.compare(other.ver) === 0;
  }

  clone(): Version {
    return new Version(this.ver);
  }

  bump(rule: BumpRule): this {
    switch (rule.type) {
      case 'major':
        this.ver = this.ver.inc('major');
        console.log(this.ver.toString());
        break;
      case 'minor':
        this.ver = this.ver.inc('minor');
        break;
      case 'patch':
        this.ver = this.ver.inc('patch');
        break;
      case 'prerelease': {
        // this.ver = new SemVer(`${this.ver.version}`)
        break;
      }
    }
    return this;
  }

  toString(): string {
    return this.ver.toString();
  }
}
