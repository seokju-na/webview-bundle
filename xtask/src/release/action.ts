import * as workspaceUtils from '../workspaceUtils';

export type Action =
  | {
      type: 'writeFile';
      filepath: string;
      content: string;
      diff: string;
    }
  | {
      type: 'addGitTag';
      tagName: string;
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
    case 'addGitTag':
      return `[ACTION] Add git tag \`${action.tagName}\``;
    case 'runCommand':
      return `[ACTION] Run command \`${action.command}\``;
  }
}
