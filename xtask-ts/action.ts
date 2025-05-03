export type Action =
  | { type: 'write'; path: string; content: string; prevContent?: string }
  | { type: 'command'; cmd: string; args: string[]; path: string };

export async function runActions(name: string, rootDir: string, actions: Action[], dryRun = false) {}
