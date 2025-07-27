import path from 'node:path';
import type { Bundle } from '@webview-bundle/node-binding';
import { app } from 'electron';
import { type Privileges, protocol } from 'electron/main';
import { lookup } from 'es-mime-types';
import type { Cache } from './cache.js';
import type { Loader } from './loader.js';
import { URI } from './uri.js';

export interface ProtocolHandlerConfig {
  loader: Loader;
  resolveUri?: (uri: URI) => string;
  cache?: Cache<Bundle>;
  contentType?: (filepath: string) => string | undefined;
  headers?: (headers: Record<string, string>) => Record<string, string>;
  error?: (e: unknown) => Response | undefined;
}

const defaultResolveUri = (uri: URI): string => uri.host;

const defaultContentType = () => {
  return (filepath: string): string | undefined => {
    const contentType: string | undefined = lookup(filepath) || undefined;
    return contentType;
  };
};

export type ProtocolHandler = (request: Request) => Promise<Response>;

export function createProtocolHandler({
  loader,
  resolveUri = defaultResolveUri,
  cache,
  contentType,
  headers,
  error,
}: ProtocolHandlerConfig): ProtocolHandler {
  const getContentType = contentType != null ? contentType : defaultContentType();
  const loadBundle = async (uri: URI): Promise<Bundle> => {
    const name = resolveUri(uri);
    if (cache?.has(name) === true) {
      return cache.get(name)!;
    }
    const bundle = await loader.load(name);
    cache?.set(name, bundle);
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
      const data = await bundle.readFileData(filepath);
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

export interface RegisterProtocolConfig extends ProtocolHandlerConfig {
  scheme: string;
  privileges?: Privileges;
}

export type UnregisterProtocol = () => void;

export function registerProtocol({ scheme, privileges, ...handlerConfig }: RegisterProtocolConfig): UnregisterProtocol {
  protocol.registerSchemesAsPrivileged([
    {
      scheme,
      privileges: {
        standard: true,
        secure: true,
        supportFetchAPI: true,
        bypassCSP: true,
        corsEnabled: true,
        codeCache: true,
        ...privileges,
      },
    },
  ]);
  app.once('ready', () => {
    const handler = createProtocolHandler(handlerConfig);
    protocol.handle(scheme, handler);
  });
  return () => {
    protocol.unhandle(scheme);
  };
}
