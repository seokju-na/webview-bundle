export interface RemoteBundleDeployment {
  /** The name of the bundle */
  name: string;
  /** Current deployed version of the bundle */
  version?: string;
  /** Versions deployed in each channel */
  channels?: Record<string, string>;
}
