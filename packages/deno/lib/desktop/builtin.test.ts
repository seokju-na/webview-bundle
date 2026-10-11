import { assert, assertEquals, assertNotEquals, assertThrows } from '@std/assert';
import { join } from '@std/path';
import { prepareBuiltin } from './builtin.ts';

function fixture() {
  const root = Deno.makeTempDirSync({ prefix: 'wvb-builtin-' });
  const builtinDir = join(root, 'embedded');
  const cacheDir = join(root, 'cache');
  const config = { builtinDir, remoteDir: join(root, 'remote') };
  Deno.mkdirSync(join(builtinDir, 'app'), { recursive: true });
  const manifest = {
    manifestVersion: 1,
    bundles: { app: { versions: { '1.0.0': {} }, currentVersion: '1.0.0' } },
  };
  Deno.writeTextFileSync(join(builtinDir, 'manifest.json'), JSON.stringify(manifest));
  const bundlePath = join(builtinDir, 'app', '1.0.0.wvb');
  Deno.writeTextFileSync(bundlePath, 'first bundle');
  return {
    root,
    config,
    cacheDir,
    bundlePath,
    manifest,
    [Symbol.dispose]() {
      Deno.removeSync(root, { recursive: true });
    },
  };
}

Deno.test('builtin extraction reuses completed files across initializations', () => {
  using f = fixture();
  using first = prepareBuiltin(f.config, f.cacheDir);
  assert(first);
  const file = join(first.builtinDir, 'app', '1.0.0.wvb');
  const timestamp = new Date('2020-01-01');
  Deno.utimeSync(file, timestamp, timestamp);
  using second = prepareBuiltin(f.config, f.cacheDir);
  assert(second);
  assertEquals(second.builtinDir, first.builtinDir);
  assertEquals(Deno.readTextFileSync(file), 'first bundle');
  assertEquals(Deno.statSync(file).mtime, timestamp);
  assertEquals(JSON.parse(Deno.readTextFileSync(second.builtinManifestFilepath)), f.manifest);
});

Deno.test('changed bytes with the same manifest produce a new extraction', () => {
  using f = fixture();
  using first = prepareBuiltin(f.config, f.cacheDir);
  assert(first);
  Deno.writeTextFileSync(f.bundlePath, 'second bundle');
  using second = prepareBuiltin(f.config, f.cacheDir);
  assert(second);
  assertNotEquals(second.builtinDir, first.builtinDir);
  assertEquals(Deno.readTextFileSync(join(first.builtinDir, 'app', '1.0.0.wvb')), 'first bundle');
  assertEquals(Deno.readTextFileSync(join(second.builtinDir, 'app', '1.0.0.wvb')), 'second bundle');
});

Deno.test('extraction honors an external manifest and does not carry removed bundles forward', () => {
  using f = fixture();
  const builtinManifestFilepath = join(f.root, 'custom.json');
  Deno.renameSync(join(f.config.builtinDir, 'manifest.json'), builtinManifestFilepath);
  const config = { ...f.config, builtinManifestFilepath };
  using first = prepareBuiltin(config, f.cacheDir);
  assert(first);
  Deno.writeTextFileSync(
    builtinManifestFilepath,
    JSON.stringify({ manifestVersion: 1, bundles: {} })
  );
  using second = prepareBuiltin(config, f.cacheDir);
  assert(second);
  assertThrows(() => Deno.statSync(join(second.builtinDir, 'app')), Deno.errors.NotFound);
  assert(Deno.statSync(join(first.builtinDir, 'app')).isDirectory);
});

Deno.test('an interrupted or partially removed extraction is repaired', () => {
  using f = fixture();
  let destination: string;
  {
    using first = prepareBuiltin(f.config, f.cacheDir);
    assert(first);
    destination = first.builtinDir;
  }
  Deno.removeSync(join(destination, 'app', '1.0.0.wvb'));
  const temporary = join(f.cacheDir, '.wvb-extract-interrupted');
  Deno.mkdirSync(temporary);
  Deno.writeTextFileSync(join(temporary, 'partial'), 'partial');
  using repaired = prepareBuiltin(f.config, f.cacheDir);
  assert(repaired);
  assertEquals(repaired.builtinDir, destination);
  assertEquals(Deno.readTextFileSync(join(destination, 'app', '1.0.0.wvb')), 'first bundle');
  assertThrows(() => Deno.statSync(temporary), Deno.errors.NotFound);
});

Deno.test('pruning keeps the previous extraction and protects active readers', () => {
  using f = fixture();
  let activePath: string;
  let previousPath: string;
  {
    using active = prepareBuiltin(f.config, f.cacheDir);
    assert(active);
    activePath = active.builtinDir;
    Deno.utimeSync(join(activePath, '.used'), 1, 1);
    Deno.writeTextFileSync(f.bundlePath, 'second');
    {
      using previous = prepareBuiltin(f.config, f.cacheDir);
      assert(previous);
      previousPath = previous.builtinDir;
      Deno.utimeSync(join(previousPath, '.used'), 2, 2);
    }
    Deno.writeTextFileSync(f.bundlePath, 'third');
    using current = prepareBuiltin(f.config, f.cacheDir);
    assert(current);
    assert(Deno.statSync(activePath).isDirectory);
    assert(Deno.statSync(previousPath).isDirectory);
  }
  using current = prepareBuiltin(f.config, f.cacheDir);
  assert(current);
  assertThrows(() => Deno.statSync(activePath), Deno.errors.NotFound);
  assert(Deno.statSync(previousPath).isDirectory);
});

Deno.test('missing builtin bundles remain optional and manifest traversal is rejected', () => {
  using f = fixture();
  assertEquals(
    prepareBuiltin({ ...f.config, builtinDir: join(f.root, 'missing') }, f.cacheDir),
    undefined
  );
  Deno.writeTextFileSync(
    join(f.config.builtinDir, 'manifest.json'),
    JSON.stringify({
      manifestVersion: 1,
      bundles: { '../escape': { versions: { '1.0.0': {} } } },
    })
  );
  assertThrows(() => prepareBuiltin(f.config, f.cacheDir), Error, 'invalid builtin manifest path');
});
