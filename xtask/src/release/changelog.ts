import { readFile } from 'node:fs/promises';

export class Changelog {
  static async load(filepath: string, isRoot = false) {
    const content = await readFile(filepath, 'utf8');
    return new Changelog(filepath, content, isRoot);
  }

  constructor(
    public readonly filepath: string,
    public content: string,
    public readonly isRoot: boolean
  ) {}
}
