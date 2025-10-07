import { Buffer } from 'node:buffer';
import {
  BundleProtocol,
  type BundleSource,
  type HttpMethod,
  type HttpResponse,
  LocalProtocol,
} from '@webview-bundle/node';
import type { Protocol as ElectronProtocol, Privileges } from 'electron';
import { app, protocol as electronProtocol } from 'electron';
import { makeError } from './utils.js';

export interface ProtocolHandler {
  handle(req: Request): Promise<Response>;
}

export interface ProtocolOptions {
  protocol?: () => ElectronProtocol;
  privileges?: Privileges;
  onError?: (e: Error) => void;
}

export interface ProtocolHandlerBuildContext {
  source: BundleSource;
}
export type ProtocolHandlerBuild = (ctx: ProtocolHandlerBuildContext) => ProtocolHandler | Promise<ProtocolHandler>;

export interface Protocol {
  scheme: string;
  handler: ProtocolHandler | ProtocolHandlerBuild;
  options?: ProtocolOptions;
}

const DEFAULT_PRIVILEGES: Privileges = {
  standard: true,
  secure: true,
  bypassCSP: true,
  allowServiceWorkers: true,
  supportFetchAPI: true,
  corsEnabled: true,
  stream: false,
  codeCache: true,
};

export async function registerProtocol(protocol: Protocol, source: BundleSource): Promise<void> {
  const { scheme, handler, options = {} } = protocol;
  const { protocol: getProtocol, privileges, onError } = options;

  electronProtocol.registerSchemesAsPrivileged([
    {
      scheme,
      privileges: { ...DEFAULT_PRIVILEGES, ...privileges },
    },
  ]);

  await app.whenReady();
  const h = typeof handler === 'function' ? await handler({ source }) : handler;
  const p = getProtocol?.() ?? electronProtocol;
  if (typeof p.handle === 'function') {
    p.handle(scheme, async req => {
      try {
        const resp = await h.handle(req);
        return resp;
      } catch (e) {
        const error = makeError(e);
        onError?.(error);
        return new Response(error.message, { status: 500 });
      }
    });
  } else {
    // support for electron < 25
    p.registerBufferProtocol(scheme, async (req, callback) => {
      const request = new Request(req.url, {
        method: req.method,
        headers: req.headers,
      });
      try {
        const response = await h.handle(request);
        callback({
          statusCode: response.status,
          headers: normalizeHeaders(response.headers),
          data: Buffer.from(await response.arrayBuffer()),
        });
      } catch (e) {
        onError?.(makeError(e));
        callback({ error: -2 });
      }
    });
  }
}

type Hosts = Record<string, string>;

export interface LocalProtocolConfig extends ProtocolOptions {
  hosts: Hosts | (() => Hosts | Promise<Hosts>);
}

export function localProtocol(scheme: string, config: LocalProtocolConfig): Protocol {
  const { hosts, ...options } = config;
  const protocol: Protocol = {
    scheme,
    handler: async () => {
      const h = typeof hosts === 'function' ? await hosts() : hosts;
      const local = new LocalProtocol(h);
      return {
        handle: async req => {
          const method = req.method.toLowerCase() as HttpMethod;
          const resp = await local.handle(method, req.url, normalizeHeaders(req.headers));
          return makeResponse(resp);
        },
      };
    },
    options,
  };
  return protocol;
}

export interface BundleProtocolConfig extends ProtocolOptions {}

export function bundleProtocol(scheme: string, config: BundleProtocolConfig = {}): Protocol {
  const { ...options } = config;
  const protocol: Protocol = {
    scheme,
    handler: ({ source }) => {
      const bundle = new BundleProtocol(source);
      return {
        handle: async req => {
          const method = req.method.toLowerCase() as HttpMethod;
          const resp = await bundle.handle(method, req.url, normalizeHeaders(req.headers));
          return makeResponse(resp);
        },
      };
    },
    options,
  };
  return protocol;
}

function normalizeHeaders(headers: Headers): Record<string, string> {
  const map: Record<string, string> = {};
  for (const [key, value] of headers.entries()) {
    map[key] = value;
  }
  return map;
}

function makeResponse(resp: HttpResponse): Response {
  const { status, headers: respHeaders, body } = resp;
  const headers = new Headers();
  for (const [key, value] of Object.entries(respHeaders)) {
    headers.set(key, value);
  }
  return new Response(body as any, { status, headers });
}
