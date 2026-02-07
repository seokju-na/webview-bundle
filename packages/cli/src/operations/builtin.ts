import fs from 'node:fs/promises';
import { type BundleManifestData, type ListRemoteBundleInfo, Remote } from '@wvb/node';
import { isRegExp } from 'es-toolkit/predicate';
import pm from 'picomatch';
import { toAbsolutePath } from '../fs.js';

type RemoteBundleMatches =
  | string
  | RegExp
  | Array<string | RegExp>
  | ((info: ListRemoteBundleInfo) => boolean | Promise<boolean>);

export interface BuiltinParams {
  remoteEndpoint: string;
  dir?: string;
  include?: RemoteBundleMatches;
  exclude?: RemoteBundleMatches;
  channel?: string;
  clean?: boolean;
  cwd?: string;
}

export async function builtin(params: BuiltinParams): Promise<BundleManifestData> {
  const { remoteEndpoint, dir: dirInput = '.wvb/builtin', include, exclude, channel, clean = true, cwd } = params;
  const dir = toAbsolutePath(dirInput, cwd);
  const manifest: BundleManifestData = {
    manifestVersion: 1,
    entries: {},
  };
  if (clean) {
    await fs.rm(dir, { recursive: true });
  }
  const remote = new Remote(remoteEndpoint);
  const remoteBundles = await remote.listBundles(channel);

  const manifest: BundleManifestData = {
    manifestVersion: 1,
    entries: {},
  };

  for (const remoteBundle of remoteBundles) {
    const shouldInclude = include != null ? await isInMatches(remoteBundle)

    const [info, , buffer] = await remote.download(remoteBundle.name, channel);

  }
}

async function isInMatches(info: ListRemoteBundleInfo, matches: RemoteBundleMatches): Promise<boolean> {
  if (typeof matches === 'function') {
    return await matches(info);
  }
  const predicates = Array.isArray(matches) ? matches : [matches];
  if (predicates.length === 0) {
    return true;
  }
  for (const predicate of predicates) {
    if (typeof predicate === 'string') {
      if (pm.isMatch(info.name, predicate)) {
        return true;
      }
    }
    if (isRegExp(predicate)) {
      if (predicate.test(info.name)) {
        return true;
      }
    }
  }
  return false;
}
