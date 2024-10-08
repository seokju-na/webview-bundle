import type { NonEmptyTuple } from 'type-fest';
import * as gitUtils from '../gitUtils';
import type { Action } from './action';
import type { ReleaseConfig } from './config';
import { type VersionedFile, loadVersionedFile } from './versionedFile';
import type { BumpRule } from './versioning';

export class Package {
  static async load(name: string, versionedFiles: string[], scopes?: string[]) {
    const loadedVersionedFiles = await Promise.all(versionedFiles.map(loadVersionedFile));
    if (loadedVersionedFiles.length === 0) {
      throw new Error(`No versioned files for package: ${name}`);
    }
    return new Package(name, loadedVersionedFiles as any as NonEmptyTuple<VersionedFile>, scopes);
  }

  static async loadAllFromConfig(config: ReleaseConfig) {
    const packages = await Promise.all(
      config.packages.entries().map(([name, info]) => Package.load(name, info.versionedFiles, info.scopes))
    );
    return packages;
  }

  protected constructor(
    public readonly name: string,
    public readonly versionedFiles: NonEmptyTuple<VersionedFile>,
    public readonly scopes?: string[]
  ) {}

  get version() {
    return this.versionedFiles[0].version;
  }

  get nextVersion() {
    return this.versionedFiles[0].nextVersion;
  }

  get versionGitTag() {
    return gitUtils.tagName(`${this.name}/v${this.version.format()}`);
  }

  get nextVersionGitTag() {
    return gitUtils.tagName(`${this.name}/v${this.nextVersion.format()}`);
  }

  bumpVersion(rule: BumpRule) {
    for (let i = 0; i < this.versionedFiles.length; i += 1) {
      this.versionedFiles[i]!.bumpVersion(rule);
    }
  }

  write(): Action[] {
    const writes = this.versionedFiles.map(x => x.write()).filter(x => x != null);
    const tags: Action[] = [
      {
        type: 'addGitTag',
        tagName: this.nextVersionGitTag,
      },
    ];
    return [...writes, ...tags];
  }
}
