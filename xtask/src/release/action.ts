import fs from 'node:fs/promises';
import * as workspaceUtils from '../workspaceUtils';

export type Action =
  | {
      type: 'writeFile';
      filepath: string;
      content: string;
      diff: string;
    }
  | {
      type: 'removeFile';
      filepath: string;
    };

export function formatAction(action: Action): string {
  switch (action.type) {
    case 'writeFile': {
      return `[ACTION] Write file to \`${workspaceUtils.relativePathToRootDir(action.filepath)}\`:
${action.diff}`;
    }
    case 'removeFile':
      return `[ACTION] Remove file \`${workspaceUtils.relativePathToRootDir(action.filepath)}\``;
  }
}

export async function runAction(action: Action) {
  switch (action.type) {
    case 'writeFile':
      await runWriteFileAction(action);
      break;
    case 'removeFile':
      await runRemoveFileAction(action);
      break;
  }
}

async function runWriteFileAction(action: Extract<Action, { type: 'writeFile' }>) {
  await fs.writeFile(action.filepath, action.content, 'utf8');
}

async function runRemoveFileAction(action: Extract<Action, { type: 'removeFile' }>) {
  await fs.rm(action.filepath, { force: true });
}
