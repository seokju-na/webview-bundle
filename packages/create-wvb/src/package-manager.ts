import fs from 'node:fs';
import { exec } from 'es-toolkit/server';

export interface PackageManager {
  name: 'npm' | 'yarn' | 'pnpm' | 'bun' | 'deno';
  version?: string;
}

export function getPackageManager(): PackageManager {
  const match = /^(npm|yarn|pnpm|bun)\/([^\s]+)/.exec(process.env.npm_config_user_agent ?? '');
  if (match == null) {
    return { name: 'npm' };
  }
  return { name: match[1] as PackageManager['name'], version: match[2] };
}

export function isYarnBerry(manager: PackageManager): boolean {
  if (manager.name !== 'yarn' || manager.version == null) {
    return false;
  }
  const major = Number.parseInt(manager.version.split('.')[0]!, 10);
  return Number.isInteger(major) && major >= 2;
}

export function runCommand(manager: PackageManager, script: string): string {
  if (manager.name === 'deno') {
    return `deno task ${script}`;
  }
  if (manager.name === 'yarn' || manager.name === 'pnpm') {
    return `${manager.name} ${script}`;
  }
  return `${manager.name} run ${script}`;
}

export async function installDependencies(manager: PackageManager, root: string): Promise<void> {
  const entry = process.env.npm_execpath;
  if (
    manager.name !== 'deno' &&
    entry != null &&
    /\.(?:c?js|mjs)$/i.test(entry) &&
    fs.existsSync(entry)
  ) {
    await exec(process.execPath, [entry, 'install'], {
      spawnOptions: { cwd: root, stdio: 'inherit' },
    });
    return;
  }
  await exec(manager.name, ['install'], {
    spawnOptions: { cwd: root, stdio: 'inherit', shell: process.platform === 'win32' },
  });
}
