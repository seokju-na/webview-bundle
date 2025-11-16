import fs from 'node:fs/promises';
import path from 'node:path';

export async function isEsmFile(filepath: string): Promise<boolean> {
  if (/\.m[jt]s$/.test(filepath)) {
    return true;
  }
  if (/\.c[jt]s$/.test(filepath)) {
    return false;
  }
  try {
    const pkg = await findNearestPackageJson(path.dirname(filepath));
    return pkg?.json.type === 'module';
  } catch {
    return false;
  }
}

export interface PackageJson {
  name?: string;
  type?: 'module' | 'commonjs';
  version?: string;
  description?: string;
  peerDependencies?: Record<string, string>;
  dependencies?: Record<string, string>;
  devDependencies?: Record<string, string>;
}

export async function findNearestPackageJson(basedir: string): Promise<{ filepath: string; json: PackageJson } | null> {
  let dir = basedir;
  while (dir) {
    const pkgJsonPath = path.join(dir, 'package.json');
    const stat = await fs.stat(pkgJsonPath).catch(() => null);
    if (stat?.isFile() === true) {
      try {
        const pkgJsonRaw = await fs.readFile(pkgJsonPath, 'utf8');
        const pkgJson = JSON.parse(pkgJsonRaw) as PackageJson;
        return { filepath: pkgJsonPath, json: pkgJson };
      } catch {}
    }
    const nextBasedir = path.dirname(dir);
    if (nextBasedir === dir) {
      break;
    }
    dir = nextBasedir;
  }
  return null;
}

export async function fileExists(filepath: string): Promise<boolean> {
  try {
    await fs.access(filepath);
    return true;
  } catch {
    return false;
  }
}
