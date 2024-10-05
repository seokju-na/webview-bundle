import type { NonEmptyTuple } from 'type-fest';
import type { Action } from './action';
import type { ReleasePackage } from './config';
import { type VersionedFile, loadVersionedFile } from './versionedFile';
import type { BumpRule } from './versioning';

export class Package {
  static async load(name: string, pkg: ReleasePackage) {
    const versionedFiles = await Promise.all(pkg.versionedFiles.map(loadVersionedFile));
    if (versionedFiles.length === 0) {
      throw new Error(`No versioned files for package: ${name}`);
    }
    return new Package(name, versionedFiles as any as NonEmptyTuple<VersionedFile>, pkg.scopes ?? []);
  }

  protected constructor(
    public readonly name: string,
    public readonly versionedFiles: NonEmptyTuple<VersionedFile>,
    public readonly scopes: string[]
  ) {}

  get version() {
    return this.versionedFiles[0].version;
  }

  get nextVersion() {
    return this.versionedFiles[0].nextVersion;
  }

  get versionDiff(): -1 | 0 | 1 {
    return this.versionedFiles[0].versionDiff;
  }

  bumpVersion(rule: BumpRule) {
    for (let i = 0; i < this.versionedFiles.length; i += 1) {
      this.versionedFiles[i]!.bumpVersion(rule);
    }
  }

  write(): Action[] {
    return this.versionedFiles.map(x => x.write()).filter(x => x != null);
  }
}
