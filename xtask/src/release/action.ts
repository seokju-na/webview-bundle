import fs from 'node:fs/promises';
import { execaCommand } from 'execa';
import * as workspaceUtils from '../workspaceUtils';

export type Action =
  | {
      type: 'writeFile';
      filepath: string;
      content: string;
      diff: string;
    }
  | {
      type: 'runCommand';
      command: string;
      cwd: string;
      env?: Record<string, string>;
    };

export function formatAction(action: Action): string {
  switch (action.type) {
    case 'writeFile': {
      return `[ACTION] Write file to \`${workspaceUtils.relativePathToRootDir(action.filepath)}\`:
${action.diff}`;
    }
    case 'runCommand':
      return `[ACTION] Run command \`${action.command}\``;
  }
}

export async function runAction(action: Action) {
  switch (action.type) {
    case 'writeFile': {
      await fs.writeFile(workspaceUtils.absolutePathFromRootDir(action.filepath), action.content, 'utf8');
      break;
    }
    case 'runCommand': {
      await execaCommand(action.command, { cwd: action.cwd, env: action.env, stdio: 'inherit' });
      break;
    }
  }
}
