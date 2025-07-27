export interface WebviewBundleConfig extends RegisterProtocolConfig, RegisterIpcConfig {}

export interface WebviewBundleOutput {
  unregisterProtocol: UnregisterProtocol;
  unregisterIpc: UnregisterIpc;
}

export function webviewBundle(config: WebviewBundleConfig): WebviewBundleOutput {
  const { loader, scheme, privileges, versionFile, resolveUri, cache, contentType, headers, error } = config;
  const unregisterProtocol = registerProtocol({
    loader,
    scheme,
    privileges,
    resolveUri,
    cache,
    contentType,
    headers,
    error,
  });
  const unregisterIpc = registerIpc({ loader, versionFile });
  return {
    unregisterProtocol,
    unregisterIpc,
  };
}
