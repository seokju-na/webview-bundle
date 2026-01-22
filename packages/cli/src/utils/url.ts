export function buildURL(endpoint: string, pathname: string): URL {
  const url = new URL(endpoint);
  const p1 = url.pathname.endsWith('/') ? url.pathname.slice(0, url.pathname.length - 1) : url.pathname;
  const p2 = pathname.startsWith('/') ? pathname.slice(1) : pathname;
  url.pathname = `${p1 === '' ? '/' : p1}${p2 === '' ? p2 : `/${p2}`}`;
  return url;
}
