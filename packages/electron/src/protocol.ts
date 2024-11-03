import path from 'node:path';
import type { Bundle } from '@webview-bundle/node-binding';
import type { lookup } from 'mime-types';
import type { Cache } from './Cache';
import type { Loader } from './Loader';
import { URI } from './URI';

export interface ProtocolHandlerConfig {
  loader: Loader;
  cache?: Cache<Bundle>;
  contentType?: (filepath: string) => string | undefined;
  headers?: (headers: Record<string, string>) => Record<string, string>;
  error?: (e: unknown) => Response | undefined;
}

const defaultContentType = () => {
  const { lookup: getContentType } = require('mime-types');
  return (filepath: string): string | undefined => {
    const contentType = (getContentType as typeof lookup)(filepath) || undefined;
    return contentType;
  };
};

export function protocolHandler({ loader, cache, contentType, headers, error }: ProtocolHandlerConfig) {
  const getContentType = contentType != null ? contentType : defaultContentType();
  const loadBundle = async (uri: URI): Promise<Bundle> => {
    const cacheKey = loader.getBundleName?.(uri) ?? uri.toString();
    if (cache?.has(cacheKey) === true) {
      return cache.get(cacheKey)!;
    }
    const bundle = await loader.load(uri);
    cache?.set(cacheKey, bundle);

    return bundle;
  };
  const makeHeaders = (filepath: string): Record<string, string> => {
    const type = getContentType(filepath);
    const defaultHeaders = {
      'content-type': type ?? 'text/plain',
    };
    return headers?.(defaultHeaders) ?? defaultHeaders;
  };

  return async (request: Request): Promise<Response> => {
    const uri = new URI(request.url);
    const bundle = await loadBundle(uri);
    const filepath = uriToFilepath(uri);
    try {
      const data = await bundle.readFile(filepath);
      const response = new Response(data, { headers: makeHeaders(filepath) });
      return response;
    } catch (e) {
      const response = error?.(e);
      if (response != null) {
        return response;
      }
      if (isFileNotFoundError(e)) {
        return new Response(null, {
          status: 404,
          headers: {
            'content-type': 'text/html; charset=utf-8',
          },
        });
      }
      throw e;
    }
  };
}

function uriToFilepath(uri: URI): string {
  const filepath = uri.path.slice(1);
  if (path.extname(filepath) !== '') {
    return decodeURIComponent(filepath);
  }
  return decodeURIComponent([filepath, 'index.html'].filter(x => x !== '').join('/'));
}

function isFileNotFoundError(e: unknown): boolean {
  return e != null && typeof e === 'object' && (e as Error)?.message?.includes('file not found');
}
