export class URI {
  readonly scheme: string;
  readonly protocol: string;
  userinfo: string;
  host: string;
  port: string;
  path: string;
  query: string;
  fragment: string;

  parse(uri: string): URI | null {
    try {
      return new URI(uri);
    } catch {
      return null;
    }
  }

  constructor(uri: string) {
    const { scheme, protocol, userinfo, host, port, path, query, fragment } = parse(uri);
    this.scheme = scheme;
    this.protocol = protocol;
    this.userinfo = userinfo;
    this.host = host;
    this.port = port;
    this.path = path;
    this.query = query;
    this.fragment = fragment;
  }

  toString(): string {
    let str = `${this.protocol}//`;
    if (this.userinfo.length > 0) {
      str += `${this.userinfo}@`;
    }
    str += this.host;
    if (this.port.length > 0) {
      str += `:${this.port}`;
    }
    str += this.path;
    if (this.query.length > 0) {
      str += `?${this.query}`;
    }
    if (this.fragment.length > 0) {
      str += `#${this.fragment}`;
    }
    return str;
  }
}

function parse(uri: string): {
  scheme: string;
  protocol: string;
  userinfo: string;
  host: string;
  port: string;
  path: string;
  query: string;
  fragment: string;
} {
  const idx = uri.indexOf('://');
  if (idx === -1) {
    throw new TypeError('URI has no valid protocol');
  }
  try {
    const scheme = uri.slice(0, idx);
    const url = new URL(uri.replace(`${scheme}://`, 'https://'));
    const userinfo = [url.username, url.password].filter(x => x !== '').join(':');
    const host = url.hostname;
    const port = url.port;
    const path =
      url.pathname === '/'
        ? url.pathname
        : url.pathname.endsWith('/')
          ? // Strip last '/'
            url.pathname.slice(0, url.pathname.length - 1)
          : url.pathname;
    const query = url.search.slice(1);
    const fragment = url.hash.slice(1);
    return {
      scheme,
      protocol: `${scheme}:`,
      userinfo,
      host,
      port,
      path,
      query,
      fragment,
    };
  } catch {
    throw new TypeError('Invalid URI');
  }
}
