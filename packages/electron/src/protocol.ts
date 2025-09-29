import { Buffer } from 'node:buffer';
import {
  type Response as BindingResponse,
  BundleProtocol,
  LocalProtocol,
  type Method,
} from '@webview-bundle/electron/binding';
import type { Protocol as ElectronProtocol, Privileges } from 'electron';
import { app, protocol as electronProtocol } from 'electron';
import { makeError } from './utils.js';

export type ProtocolHandler = (req: Request) => Promise<Response>;

export interface ProtocolOptions {
  protocol?: () => ElectronProtocol;
  privileges?: Privileges;
  onError?: (e: Error) => void;
}

export interface Protocol {
  scheme: string;
  handler: ProtocolHandler;
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

export async function registerProtocol(protocol: Protocol): Promise<void> {
  const { scheme, handler, options = {} } = protocol;
  const { protocol: getProtocol, privileges, onError } = options;

  electronProtocol.registerSchemesAsPrivileged([
    {
      scheme,
      privileges: { ...DEFAULT_PRIVILEGES, ...privileges },
    },
  ]);

  await app.whenReady();
  const p = getProtocol?.() ?? electronProtocol;
  if (typeof p.handle === 'function') {
    p.handle(scheme, async req => {
      try {
        const resp = await handler(req);
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
        const response = await handler(request);
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

export function localProtocol(scheme: string, mapping: Record<string, string>, options?: ProtocolOptions): Protocol {
  const local = new LocalProtocol(mapping);
  const protocol: Protocol = {
    scheme,
    handler: async req => {
      const method = req.method.toLowerCase() as Method;
      const resp = await local.handle(method as Method, req.url, normalizeHeaders(req.headers));
      return makeResponse(resp);
    },
    options,
  };
  return protocol;
}

export function bundleProtocol(scheme: string, basedir: string, options?: ProtocolOptions): Protocol {
  const bundle = new BundleProtocol(basedir);
  const protocol: Protocol = {
    scheme,
    handler: async req => {
      const method = req.method.toLowerCase() as Method;
      const resp = await bundle.handle(method as Method, req.url, normalizeHeaders(req.headers));
      return makeResponse(resp);
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

function makeResponse(resp: BindingResponse): Response {
  const { status, headers: respHeaders, body } = resp;
  const headers = new Headers();
  for (const [key, value] of Object.entries(respHeaders)) {
    headers.set(key, value);
  }
  return new Response(body as any, { status, headers });
}
