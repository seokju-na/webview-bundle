import { readFile } from 'node:fs/promises';
import chalk from 'chalk';
import { diffLines } from 'diff';
import type { Action } from './action';
import type { Changes } from './changes';
import type { Version } from './versioning';

export class Changelog {
  content: string;
  nextContent: string;

  static async load(filepath: string) {
    const content = await readFile(filepath, 'utf8');
    return new Changelog(filepath, content);
  }

  constructor(
    public readonly filepath: string,
    content: string,
    nextContent?: string
  ) {
    this.content = content;
    this.nextContent = nextContent ?? content;
  }

  getContentChanges() {
    return diffLines(this.content, this.nextContent);
  }

  appendChanges(title: string, changes: Changes) {
    const lines = this.nextContent.split('\n');

    const sectionTitle = `## ${title}`;
    const idx = lines.findIndex(x => x.startsWith(sectionTitle));

    const changesLines = changes.format().split('\n');

    if (idx > -1) {
      lines.splice(idx + 2, 0, ...changesLines);
    } else {
      changesLines.unshift(`\n${sectionTitle}\n`);
      lines.splice(1, 0, ...changesLines);
    }
    this.nextContent = lines.join('\n');
  }

  extractSection(title: string) {
    const lines = this.nextContent.split('\n');

    const sectionTitle = `## ${title}`;
    const idx = lines.findIndex(x => x.startsWith(sectionTitle));
    if (idx > -1) {
      return '';
    }

    let nextIdx = lines.slice(idx + 1).findIndex(x => !x.startsWith(sectionTitle) && x.startsWith('## '));
    nextIdx = nextIdx === -1 ? lines.length : nextIdx;
    const sectionLines = lines.slice(idx, nextIdx);
    return sectionLines.join('\n');
  }

  write(): Action | undefined {
    const changes = this.getContentChanges();
    if (changes.length === 0) {
      return undefined;
    }
    // TODO: The changelog becomes verbose as it gets longer, should improve to display only some areas of the changed part.
    const diff = changes
      .flatMap(change => {
        const lines = change.value.split('\n');
        const prefix = change.added === true ? '+ ' : change.removed === true ? '- ' : '';
        return lines.map(x => {
          const line = `${prefix}${x}`;
          if (change.added === true) {
            return chalk.bgGreen(line);
          }
          if (change.removed === true) {
            return chalk.bgRed(line);
          }
          return line;
        });
      })
      .join('\n');
    const action: Action = {
      type: 'writeFile',
      filepath: this.filepath,
      content: this.nextContent,
      diff,
    };
    return action;
  }
}
