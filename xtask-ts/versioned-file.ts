import fs from 'node:fs/promises';
import path from 'node:path';
import * as TOML from '@ltd/j-toml';
import * as toml from '@std/toml';
import taploLib from '@taplo/lib';
import { diffLines } from 'diff';
import glob from 'fast-glob';
import type { PackageJson as PackageJsonType } from 'type-fest';
import type { Action } from './action.ts';
import { ROOT_DIR } from './consts.ts';
import { type BumpRule, Version } from './version.ts';

export type VersionedFileType = 'Cargo.toml' | 'package.json';

export class VersionedFile {
  readonly type: VersionedFileType;
  private _nextVersion: Version | null;
  private pkgManager: PackageManager;

  static async loadAll(patterns: string[]): Promise<VersionedFile[]> {
    const files = await glob(patterns, {
      cwd: ROOT_DIR,
      onlyFiles: true,
    });
    const versionedFiles = await Promise.all(files.map(filepath => VersionedFile.load(filepath)));
    return versionedFiles;
  }

  static async load(filepath: string): Promise<VersionedFile> {
    const absolutePath = path.join(ROOT_DIR, filepath);
    const filename = path.basename(absolutePath);
    const content = await fs.readFile(absolutePath, 'utf8');
    switch (filename) {
      case 'package.json':
        return new VersionedFile('package.json', new PackageJson(filepath, content));
      case 'Cargo.toml':
        return new VersionedFile('Cargo.toml', new Cargo(filepath, content));
      default:
        throw new Error(`unrecognized file: ${filepath}`);
    }
  }

  constructor(type: VersionedFileType, pkgManager: PackageManager) {
    this.type = type;
    this._nextVersion = null;
    this.pkgManager = pkgManager;
  }

  get name(): string {
    return this.pkgManager.name;
  }

  get path(): string {
    return this.pkgManager.path;
  }

  get version(): Version {
    return this.pkgManager.version.clone();
  }

  get nextVersion(): Version {
    if (this._nextVersion == null) {
      return this.pkgManager.version.clone();
    }
    return this._nextVersion.clone();
  }

  get hasChanged(): boolean {
    return !this.version.equals(this.nextVersion);
  }

  bumpVersion(rule: BumpRule): void {
    this._nextVersion = this.version;
    this._nextVersion.bump(rule);
  }

  async write(): Promise<Action[]> {
    if (!this.hasChanged) {
      return [];
    }
    return await this.pkgManager.write(this.nextVersion);
  }

  async publish(): Promise<Action[]> {
    if (!this.hasChanged) {
      return [];
    }
    return await this.pkgManager.publish(this.nextVersion);
  }
}

interface PackageManager {
  readonly name: string;
  readonly path: string;
  readonly version: Version;
  readonly canPublish: boolean;
  write(nextVersion: Version): Promise<Action[]>;
  publish(nextVersion: Version): Promise<Action[]>;
}

class PackageJson implements PackageManager {
  private readonly json: PackageJsonType;
  private readonly _path: string;
  private readonly raw: string;

  constructor(path: string, raw: string) {
    const parsed: PackageJsonType = JSON.parse(raw);
    if (parsed.name == null) {
      throw new Error('"name" field is required in package.json');
    }
    if (parsed.version == null) {
      throw new Error('"version" field is required in package.json');
    }
    this.json = parsed;
    this._path = path;
    this.raw = raw;
  }

  get name(): string {
    return this.json.name!;
  }

  get path(): string {
    return this._path;
  }

  get version(): Version {
    return Version.parse(this.json.version!);
  }

  get canPublish(): boolean {
    return this.json.private !== true;
  }

  async write(nextVersion: Version): Promise<Action[]> {
    const json = { ...this.json };
    json.version = nextVersion.toString();

    const content = `${JSON.stringify(json, null, 2)}\n`;
    return [
      {
        type: 'write',
        path: this.path,
        content,
        prevContent: this.raw,
      },
    ];
  }

  async publish(nextVersion: Version): Promise<Action[]> {
    const args = ['npm', 'publish', '--access', 'public', '--provenance', 'true'];
    const prerelease = nextVersion.prerelease;
    if (prerelease != null) {
      args.push('--tag', prerelease.id);
    }
    return [
      {
        type: 'command',
        cmd: 'yarn',
        args,
        path: this.path,
      },
    ];
  }
}

class Cargo implements PackageManager {
  private readonly toml: any;
  private readonly _path: string;
  private readonly raw: string;

  constructor(_path: string, raw: string) {
    const parsed: any = toml.parse(raw);
    if (parsed.package?.name == null) {
      throw new Error('"name" field is required in Cargo.toml');
    }
    if (parsed.package?.version == null) {
      throw new Error('"version" field is required in Cargo.toml');
    }
    this.toml = parsed;
    this._path = _path;
    this.raw = raw;
  }

  get name(): string {
    return this.toml.package.name;
  }

  get path(): string {
    return this._path;
  }

  get version(): Version {
    return Version.parse(this.toml.package.version);
  }

  get canPublish(): boolean {
    return this.toml.package?.publish !== false;
  }

  async write(nextVersion: Version): Promise<Action[]> {
    const version = nextVersion.toString();

    // const configRaw = await fs.readFile(path.join(ROOT_DIR, 'taplo.toml'), 'utf8');
    // let config: any = toml.parse(configRaw);
    // config = mapValues(config, val => {
    //   if (typeof val === 'object' && val != null && !Array.isArray(val)) {
    //     console.log(val);
    //     return mapKeys(val, (_, key) => camelCase(key));
    //   }
    //   return val;
    // });
    // console.log(config);
    const taplo = await taploLib.Taplo.initialize();
    // const Taplo = new taploLib.Taplo;

    // const table = TOML.parse(this.raw);
    // console.log(table);

    const edited: any = TOML.parse(this.raw);
    edited.package.version = version;

    if (edited.dependencies?.[this.name] != null) {
      if (typeof edited.dependencies[this.name] === 'string') {
        edited.dependencies[this.name] = version;
      } else if (typeof edited.dependencies[this.name]?.version === 'string') {
        edited.dependencies[this.name].version = version;
      }
    }

    if (edited['dev-dependencies']?.[this.name] != null) {
      if (typeof edited['dev-dependencies'][this.name] === 'string') {
        edited['dev-dependencies'][this.name] = version;
      } else if (typeof edited['dev-dependencies'][this.name]?.version === 'string') {
        edited['dev-dependencies'][this.name].version = version;
      }
    }

    if (edited.workspace?.dependencies?.[this.name] != null) {
      if (typeof edited.workspace?.dependencies?.[this.name] === 'string') {
        edited.workspace.dependencies[this.name] = version;
      } else if (typeof edited.workspace.dependencies[this.name]?.version === 'string') {
        edited.workspace.dependencies[this.name].version = version;
      }
    }

    let content = TOML.stringify(edited) as any as string[];
    content = content.slice(1);
    content = content.filter((line, i) => {
      const prevLine = content[i - 1];
      return !(prevLine?.startsWith('[') === true && line === '');
    });
    const formatted = taplo
      .format(content.join('\n'), {
        options: {
          alignEntries: true,
          columnWidth: 120,
          reorderKeys: true,
        },
      })
      .replaceAll(/'/g, '"');

    const diff = diffLines(this.raw, formatted);
    console.log(diff.filter(x => x.added || x.removed));

    return [
      {
        type: 'write',
        path: this.path,
        content: formatted,
        prevContent: this.raw,
      },
    ];
  }

  async publish(_nextVersion: Version): Promise<Action[]> {
    const args = ['publish', '--allow-dirty'];
    return [
      {
        type: 'command',
        cmd: 'cargo',
        args,
        path: this.path,
      },
    ];
  }
}
