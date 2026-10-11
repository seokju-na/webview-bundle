import { createHash } from 'node:crypto';
import { dirname, join } from '@std/path';
import type { ManifestData, SourceConfig } from '../source.ts';

interface BundleFile {
  relativePath: string;
  size: number;
  checksum: string;
}

export interface PreparedBuiltin extends Disposable {
  builtinDir: string;
  builtinManifestFilepath: string;
}

const activeBuiltins = new Map<string, PreparedBuiltin>();

/** Keep compiled-app resources locked until the process exits, including when a Source escapes. */
export function retainBuiltin(prepared: PreparedBuiltin): void {
  if (activeBuiltins.has(prepared.builtinDir)) {
    prepared[Symbol.dispose]();
  } else {
    activeBuiltins.set(prepared.builtinDir, prepared);
  }
}

/** Materialize VFS bundles before their paths are passed to Rust. The caller owns the lease. */
export function prepareBuiltin(
  config: SourceConfig,
  cacheDir: string
): PreparedBuiltin | undefined {
  let manifestBytes: Uint8Array;
  try {
    manifestBytes = Deno.readFileSync(
      config.builtinManifestFilepath ?? join(config.builtinDir, 'manifest.json')
    );
  } catch (error) {
    if (error instanceof Deno.errors.NotFound && config.builtinManifestFilepath == null) {
      return undefined;
    }
    throw error;
  }
  const manifest = JSON.parse(new TextDecoder().decode(manifestBytes)) as ManifestData;
  const files: BundleFile[] = [];
  for (const [name, bundle] of Object.entries(manifest.bundles)) {
    checkSegment(name);
    for (const version of Object.keys(bundle.versions)) {
      checkSegment(version);
      const relativePath = `${name}/${version}.wvb`;
      files.push({ relativePath, ...hashFile(join(config.builtinDir, relativePath)) });
    }
  }
  files.sort((a, b) => a.relativePath.localeCompare(b.relativePath));
  const hash = createHash('sha256')
    .update(manifestBytes)
    .update(JSON.stringify(files))
    .digest('hex');
  const destination = join(cacheDir, hash);
  Deno.mkdirSync(cacheDir, { recursive: true });
  using lock = Deno.openSync(join(cacheDir, '.wvb-extraction.lock'), {
    read: true,
    write: true,
    create: true,
  });
  lock.lockSync(true);

  if (!isComplete(destination, hash, files)) {
    if (exists(destination)) {
      if (!removeUnused(destination)) {
        throw new Error(`wvb: builtin cache is incomplete but still in use: ${destination}`);
      }
    }
    const temporary = Deno.makeTempDirSync({ dir: cacheDir, prefix: '.wvb-extract-' });
    try {
      for (const file of files) {
        const target = join(temporary, file.relativePath);
        Deno.mkdirSync(dirname(target), { recursive: true });
        using output = Deno.openSync(target, { write: true, createNew: true });
        const copied = hashFile(join(config.builtinDir, file.relativePath), output);
        if (copied.checksum !== file.checksum || copied.size !== file.size) {
          throw new Error(`wvb: builtin changed during extraction: ${file.relativePath}`);
        }
      }
      Deno.writeFileSync(join(temporary, 'manifest.json'), manifestBytes);
      Deno.writeTextFileSync(join(temporary, '.complete'), hash);
      Deno.renameSync(temporary, destination);
    } finally {
      if (exists(temporary)) {
        Deno.removeSync(temporary, { recursive: true });
      }
    }
  }

  const lease = Deno.openSync(join(destination, '.lease'), {
    read: true,
    write: true,
    create: true,
  });
  try {
    lease.lockSync();
    Deno.writeTextFileSync(join(destination, '.used'), '');
    prune(cacheDir, hash);
  } catch (error) {
    lease.close();
    throw error;
  }
  return {
    builtinDir: destination,
    builtinManifestFilepath: join(destination, 'manifest.json'),
    [Symbol.dispose]: () => lease.close(),
  };
}

function checkSegment(value: string): void {
  if (value === '' || value === '.' || value === '..' || /[/\\:\0]/.test(value)) {
    throw new Error(`wvb: invalid builtin manifest path segment: ${JSON.stringify(value)}`);
  }
}

function hashFile(path: string, output?: Deno.FsFile): { size: number; checksum: string } {
  using input = Deno.openSync(path);
  const hash = createHash('sha256');
  const buffer = new Uint8Array(64 * 1024);
  let size = 0;
  for (let count = input.readSync(buffer); count !== null; count = input.readSync(buffer)) {
    const bytes = buffer.subarray(0, count);
    hash.update(bytes);
    size += count;
    if (output != null) {
      let written = 0;
      while (written < count) {
        written += output.writeSync(bytes.subarray(written));
      }
    }
  }
  return { size, checksum: hash.digest('hex') };
}

function isComplete(dir: string, hash: string, files: BundleFile[]): boolean {
  try {
    return (
      Deno.readTextFileSync(join(dir, '.complete')) === hash &&
      createHash('sha256')
        .update(Deno.readFileSync(join(dir, 'manifest.json')))
        .update(JSON.stringify(files))
        .digest('hex') === hash &&
      files.every(file => {
        const stat = Deno.statSync(join(dir, file.relativePath));
        return stat.isFile && stat.size === file.size;
      })
    );
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) {
      return false;
    }
    throw error;
  }
}

function exists(path: string): boolean {
  try {
    Deno.statSync(path);
    return true;
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) {
      return false;
    }
    throw error;
  }
}

function removeUnused(dir: string): boolean {
  const lease = Deno.openSync(join(dir, '.lease'), { read: true, write: true, create: true });
  let unused: boolean;
  try {
    unused = lease.tryLockSync(true);
  } finally {
    lease.close();
  }
  // The cache-wide lock prevents new readers between releasing the lease and removal on Windows.
  if (unused) {
    Deno.removeSync(dir, { recursive: true });
  }
  return unused;
}

function prune(cacheDir: string, current: string): void {
  const previous: Array<{ path: string; used: number }> = [];
  for (const entry of Deno.readDirSync(cacheDir)) {
    if (!entry.isDirectory || entry.name === current) {
      continue;
    }
    const path = join(cacheDir, entry.name);
    if (entry.name.startsWith('.wvb-extract-')) {
      Deno.removeSync(path, { recursive: true });
    } else if (/^[a-f0-9]{64}$/.test(entry.name)) {
      let used = 0;
      try {
        used = Deno.statSync(join(path, '.used')).mtime?.getTime() ?? 0;
      } catch (error) {
        if (!(error instanceof Deno.errors.NotFound)) {
          throw error;
        }
      }
      previous.push({ path, used });
    }
  }
  previous.sort((a, b) => b.used - a.used);
  for (const { path } of previous.slice(1)) {
    removeUnused(path);
  }
}
