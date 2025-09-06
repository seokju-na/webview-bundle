export function intoHeaders(value: Array<[string, string]>): Headers {
  const headers = new Headers();
  for (const [key, val] of value) {
    headers.append(key, val);
  }
  return headers;
}
