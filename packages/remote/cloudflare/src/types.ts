export interface RemoteBundleDeployment {
  /** The name of the bundle */
  name: string;
  /** Current deployed version of the bundle */
  version?: string;
  /** Versions deployed in each channel */
  channels?: Record<string, string>;
}

export function getRemoteBundleDeploymentVersion(
  deployment: RemoteBundleDeployment,
  channel?: string
): string | undefined {
  if (channel != null && deployment.channels?.[channel] != null) {
    return deployment.channels[channel];
  }
  return deployment.version;
}
