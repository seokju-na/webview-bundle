import { parse, type SemVer } from 'semver';
import { z } from 'zod';

export const PrereleaseDataSchema = z.object({
  id: z.string(),
  num: z.number(),
});
export type PrereleaseData = z.infer<typeof PrereleaseDataSchema>;

export function parsePrerelease(x: unknown): PrereleaseData {
  if (typeof x !== 'string') {
    throw new Error(`invalid prerelease: ${String(x)}`);
  }
  const [id, num] = x.split('.');
  if (id == null || x.length === 0) {
    throw new Error(`invalid prerelease: ${x}`);
  }
  if (num === '' || Number.isNaN(Number(num))) {
    throw new Error(`invalid prerelease: ${x}`);
  }
  return { id, num: Number(num) };
}

export const BumpRuleSchema = z.discriminatedUnion('type', [
  z.object({ type: z.literal('major') }),
  z.object({ type: z.literal('minor') }),
  z.object({ type: z.literal('patch') }),
  z.object({ type: z.literal('prerelease'), data: PrereleaseDataSchema }),
]);
export type BumpRule = z.infer<typeof BumpRuleSchema>;

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

  get prerelease(): PrereleaseData | null {
    if (this.ver.prerelease.length < 2) {
      return null;
    }
    try {
      return parsePrerelease(`${this.ver.prerelease[0]}.${this.ver.prerelease[1]}`);
    } catch {
      return null;
    }
  }

  equals(other: Version): boolean {
    return this.ver.compare(other.ver) === 0;
  }

  clone(): Version {
    return Version.parse(this.ver.toString());
  }

  bump(rule: BumpRule): this {
    switch (rule.type) {
      case 'major':
        this.ver = this.ver.inc('major');
        break;
      case 'minor':
        this.ver = this.ver.inc('minor');
        break;
      case 'patch':
        this.ver = this.ver.inc('patch');
        break;
      case 'prerelease': {
        const ver = `${this.ver.major}.${this.ver.minor}.${this.ver.patch}-${rule.data.id}.${rule.data.num}`;
        const parsed = parse(ver);
        if (parsed == null) {
          throw new Error(`invalid version: ${ver}`);
        }
        this.ver = parsed;
        break;
      }
    }
    return this;
  }

  toString(): string {
    return this.ver.toString();
  }
}
