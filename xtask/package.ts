import type { ReadonlyDeep } from 'type-fest';
import type { Action } from './action.ts';
import type { Config, PackageConfig } from './config.ts';
import type { BumpRule, Version } from './version.ts';
import { VersionedFile } from './versioned-file.ts';
import { VersionedGitTag } from './versioned-git-tag.ts';

type NonEmptyArray<T> = readonly [T, ...T[]];

function isNonEmptyArray<T>(x: readonly T[]): x is NonEmptyArray<T> {
  return x.length > 0;
}

export class Package {
  public readonly name: string;
  public readonly versionedFiles: NonEmptyArray<VersionedFile>;
  public readonly config: ReadonlyDeep<PackageConfig>;

  static async loadAll(config: Config): Promise<Package[]> {
    const packages: Package[] = [];
    for (const [name, pkg] of Object.entries(config.packages)) {
      const versionedFiles = await VersionedFile.loadAll(pkg.versionedFiles);
      if (!isNonEmptyArray(versionedFiles)) {
        throw new Error(`"versionedFiles" must not be empty: ${name}`);
      }
      packages.push(new Package(name, versionedFiles, pkg));
    }
    return packages;
  }

  constructor(name: string, versionedFiles: NonEmptyArray<VersionedFile>, config: PackageConfig) {
    this.name = name;
    this.versionedFiles = versionedFiles;
    this.config = config;
  }

  get version(): Version {
    return this.versionedFiles[0].version;
  }

  get nextVersion(): Version {
    return this.versionedFiles[0].nextVersion;
  }

  get hasChanged(): boolean {
    return this.versionedFiles[0].hasChanged;
  }

  get versionedGitTag(): VersionedGitTag {
    return new VersionedGitTag(this.name, this.version);
  }

  get nextVersionedGitTag(): VersionedGitTag {
    return new VersionedGitTag(this.name, this.nextVersion);
  }

  bumpVersion(rule: BumpRule): this {
    for (const versionedFile of this.versionedFiles) {
      versionedFile.bumpVersion(rule);
    }
    return this;
  }

  write(): Action[] {
    return this.versionedFiles.flatMap(x => x.write());
  }

  publish(): Action[] {
    return this.versionedFiles.flatMap(x => x.publish());
  }
}
