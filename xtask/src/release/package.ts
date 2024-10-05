import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import type { PackageJson as PackageJsonType } from 'type-fest';
import toml from '@taplo/lib';
import { type BumpRule, Version } from './versioning';

export type PackageType = 'package.json' | 'Cargo.toml';

export interface Package {
  readonly type: PackageType;
  readonly filepath: string;
  version: Version;
  nextVersion: Version;
  get diff(): -1 | 0 | 1;
  bump(rule: BumpRule): void;
  write(): Promise<boolean>;
}

class PackageJson implements Package {
  readonly type: PackageType = 'package.json';
  private readonly raw: PackageJsonType;
  version: Version;
  nextVersion: Version;

  static async load(filepath: string) {
    const raw: PackageJsonType = JSON.parse(await readFile(filepath, 'utf8'));
    const version = Version.parse(raw.version!);
    return new PackageJson(filepath, raw, version);
  }

  constructor(
    public readonly filepath: string,
    raw: PackageJsonType,
    version: Version,
    nextVersion?: Version
  ) {
    this.version = version;
    this.nextVersion = nextVersion ?? version;
    this.raw = raw;
  }

  get diff() {
    return this.nextVersion.compare(this.version);
  }

  bump(rule: BumpRule) {
    this.nextVersion = this.version.bump(rule);
  }

  async write() {
    if (this.diff === 0) {
      return false;
    }
    const raw = { ...this.raw, version: this.nextVersion.format() };
    await writeFile(this.filepath, JSON.stringify(raw, null, 2));
    return true;
  }
}

class CargoTomlPackage implements Package {
  readonly type: PackageType = 'Cargo.toml';

  static async load(filepath: string) {

  }
}

export async function loadPackage(filepath: string): Promise<Package> {
  const filename = path.basename(filepath);
  switch (filename) {
    case 'package.json':
      return await PackageJson.load(filepath);
    case 'Cargo.toml':
      break;
    default:
      throw new Error(`Invalid package: ${filename}`);
  }
}
