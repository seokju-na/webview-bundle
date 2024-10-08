import { readFile } from 'node:fs/promises';
import path from 'node:path';
import init, { edit, parse } from '@rainbowatcher/toml-edit-js';
import chalk from 'chalk';
import { execa } from 'execa';
import type { PackageJson as PackageJsonType } from 'type-fest';
import type { Action } from './action';
import { type BumpRule, Version } from './versioning';

export type VersionedFileType = 'package.json' | 'Cargo.toml';

export interface VersionedFile {
  readonly type: VersionedFileType;
  readonly filepath: string;
  version: Version;
  nextVersion: Version;
  get versionChanged(): boolean;
  bumpVersion(rule: BumpRule): void;
  write(): Action | undefined;
  publish(dryRun?: boolean): Promise<boolean>;
}

abstract class BaseVersionedFile implements VersionedFile {
  version: Version;
  nextVersion: Version;

  protected constructor(
    public readonly type: VersionedFileType,
    public readonly filepath: string,
    version: Version,
    nextVersion?: Version
  ) {
    this.version = version;
    this.nextVersion = nextVersion ?? version;
  }

  get versionChanged(): boolean {
    return this.nextVersion.compare(this.version) !== 0;
  }

  bumpVersion(rule: BumpRule) {
    this.nextVersion = this.version.bump(rule);
  }

  abstract write(): Action | undefined;
  abstract publish(dryRun?: boolean): Promise<boolean>;
}

class PackageJsonVersionedFile extends BaseVersionedFile {
  private readonly raw: PackageJsonType;

  static async load(filepath: string) {
    const raw: PackageJsonType = JSON.parse(await readFile(filepath, 'utf8'));
    const version = Version.parse(raw.version ?? '0.0.0');
    return new PackageJsonVersionedFile(raw, filepath, version);
  }

  protected constructor(raw: PackageJsonType, filepath: string, version: Version, nextVersion?: Version) {
    super('package.json', filepath, version, nextVersion);
    this.raw = raw;
  }

  write(): Action | undefined {
    if (!this.versionChanged) {
      return undefined;
    }
    const raw = { ...this.raw, version: this.nextVersion.format() };
    const diff = [
      chalk.bgRed(`- "version": "${this.version.format()}"`),
      chalk.bgGreen(`+ "version": "${this.nextVersion.format()}"`),
    ].join('\n');
    const action: Action = {
      type: 'writeFile',
      filepath: this.filepath,
      content: `${JSON.stringify(raw, null, 2)}\n`,
      diff,
    };
    return action;
  }

  async publish(dryRun = false) {
    if (this.raw.private === true) {
      return false;
    }
    const cwd = path.basename(this.filepath);
    const args = ['npm', 'publish', '--access', 'public'];
    if (dryRun) {
      args.push('--dry-run');
    }
    await execa('yarn', args, { stdio: 'inherit', cwd });
    return true;
  }
}

class CargoTomlVersionedFile extends BaseVersionedFile {
  private readonly raw: string;

  static async load(filepath: string) {
    const wasmFilepath = path.join(require.resolve('@rainbowatcher/toml-edit-js'), '../index.wasm');
    await init({ module_or_path: wasmFilepath });
    const raw = await readFile(filepath, 'utf8');
    const version = Version.parse(parse(raw)?.package?.version ?? '0.0.0');
    return new CargoTomlVersionedFile(raw, filepath, version);
  }

  protected constructor(raw: string, filepath: string, version: Version, nextVersion?: Version) {
    super('Cargo.toml', filepath, version, nextVersion);
    this.raw = raw;
  }

  write(): Action | undefined {
    if (!this.versionChanged) {
      return undefined;
    }
    const content = edit(this.raw, 'package.version', this.nextVersion.format());
    const diff = [
      chalk.bgRed(`- version = "${this.version.format()}"`),
      chalk.bgGreen(`+ version = "${this.nextVersion.format()}"`),
    ].join('\n');
    const action: Action = {
      type: 'writeFile',
      filepath: this.filepath,
      content,
      diff,
    };
    return action;
  }

  async publish(dryRun = false) {
    const parsed = parse(this.raw);
    if (parsed?.package?.publish === false) {
      return false;
    }
    const cwd = path.basename(this.filepath);
    const args = ['publish'];
    if (dryRun) {
      args.push('--dry-run');
    }
    await execa('cargo', args, { stdio: 'inherit', cwd });
    return true;
  }
}

export async function loadVersionedFile(filepath: string): Promise<VersionedFile> {
  const filename = path.basename(filepath);
  switch (filename) {
    case 'package.json':
      return await PackageJsonVersionedFile.load(filepath);
    case 'Cargo.toml':
      return await CargoTomlVersionedFile.load(filepath);
    default:
      throw new Error(`Invalid package: ${filename}`);
  }
}
