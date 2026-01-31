export function getBundleDownloadPath(bundleName: string, version: string): string {
  return `bundles/${bundleName}/${version}/${bundleName}_${version}.wvb`;
}
