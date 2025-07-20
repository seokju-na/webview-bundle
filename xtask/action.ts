import fs from 'node:fs/promises';
import path from 'node:path';
import { diffLines } from 'diff';
import { runCommand } from './child_process.ts';
import { colors } from './console.ts';
import { ROOT_DIR } from './consts.ts';

export type Action =
  | { type: 'write'; path: string; content: string; prevContent?: string }
  | { type: 'command'; cmd: string; args: string[]; path: string; ignoreError?: boolean };

interface ActionContext {
  name?: string;
  dryRun: boolean;
}

export async function runActions(actions: Action[], ctx: ActionContext) {
  if (ctx.dryRun) {
    dryRunActions(actions, ctx);
    return;
  }
  for (const action of actions) {
    switch (action.type) {
      case 'write':
        await runWriteAction(action, ctx);
        break;
      case 'command':
        await runCommandAction(action, ctx);
        break;
    }
  }
}

async function runWriteAction(action: Extract<Action, { type: 'write' }>, { name = 'root' }: ActionContext) {
  const filepath = path.join(ROOT_DIR, action.path);
  await fs.mkdir(path.dirname(filepath), { recursive: true });
  await fs.writeFile(filepath, action.content, 'utf8');
  console.log(`${colors.success(`[${name}]`)} write file: ${action.path}`);
  if (action.prevContent != null) {
    logDiff(action.prevContent, action.content);
  }
}

async function runCommandAction(action: Extract<Action, { type: 'command' }>, { name = 'root' }: ActionContext) {
  console.log(`${colors.info(`[${name}]`)} run command: ${action.cmd} ${action.args.join(' ')}`);
  console.log(`  ${colors.dim(`path: ${action.path}`)}`);
  const { exitCode } = await runCommand(action.cmd, action.args, {
    cwd: path.join(ROOT_DIR, action.path),
    prefix: `${colors.info(`[${name}}`)} `,
  });
  if (action.ignoreError !== true && exitCode !== 0) {
    throw new Error(`[${name}] command failed with exit code: ${exitCode}`);
  }
}

function dryRunActions(actions: Action[], ctx: ActionContext) {
  for (const action of actions) {
    dryRunAction(action, ctx);
  }
}

function dryRunAction(action: Action, ctx: ActionContext) {
  switch (action.type) {
    case 'write':
      dryRunWriteAction(action, ctx);
      break;
    case 'command':
      dryRunCommandAction(action, ctx);
      break;
  }
}

function dryRunWriteAction(action: Extract<Action, { type: 'write' }>, { name = 'root' }: ActionContext) {
  const { path, content, prevContent } = action;
  const prefix = colors.info(`[${name}]`);
  console.log(`${prefix} will write file: ${path}`);
  if (prevContent != null) {
    logDiff(prevContent, content);
  }
}

function logDiff(prevContent: string, content: string) {
  const diff = diffLines(prevContent, content);
  let modified: number | undefined;
  let lineNo = 0;
  for (const change of diff) {
    if (!change.added && !change.removed) {
      if (modified != null) {
        lineNo += modified;
        modified = undefined;
      }
      lineNo += change.count ?? 0;
      continue;
    }
    let changeLineNo = lineNo;
    const lines = change.value.trimEnd().split('\n');
    for (const line of lines) {
      changeLineNo += 1;
      const lineStr = String(changeLineNo).padStart(3, ' ');
      const color = change.added ? colors.success : colors.error;
      const diffPrefix = change.added ? '+' : '-';
      console.log(`  ${colors.dim(lineStr)}|${diffPrefix}${color(line)}`);
    }
    modified = change.count;
  }
}

function dryRunCommandAction(action: Extract<Action, { type: 'command' }>, { name = 'root' }: ActionContext) {
  const { path, cmd, args } = action;
  const prefix = colors.info(`[${name}]`);
  console.log(`${prefix} will run command: ${cmd} ${args.join(' ')}`);
  console.log(`  ${colors.dim(`path: ${path}`)}`);
}
