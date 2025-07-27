export type Platform = 'electron' | 'tauri';

export interface UserAgentInfo {
  version: string;
  platform: Platform;
  platformVersion: string;
  os?: string;
  arch?: string;
}

const regexp =
  /WebViewBundle\/([^\s(]+)\s*\(\s*([^\s]+)\s+([\d.]+(?:\.\d+)*(?:-[a-zA-Z0-9]+(?:\.[a-zA-Z0-9]+)*)?(?:\+[a-zA-Z0-9]+(?:\.[a-zA-Z0-9]+)*)?)(?:\s*;\s*([^;)]+))?(?:\s*;\s*([^;)]+))?\s*\)/i;

export function parseUserAgent(userAgent: string): UserAgentInfo | null {
  const [, version, platformRaw, platformVersion, os, arch] = userAgent.match(regexp) ?? [];
  if (version == null || platformRaw == null || platformVersion == null) {
    return null;
  }
  const platform = parsePlatform(platformRaw);
  if (platform == null) {
    return null;
  }
  const info: UserAgentInfo = {
    version,
    platform,
    platformVersion,
    os: parseOs(os) ?? undefined,
    arch: parseArch(arch) ?? undefined,
  };
  return info;
}

function parseOs(os: unknown): string | null {
  if (typeof os === 'string') {
    const val = os.trim();
    if (val.length === 0) {
      return null;
    }
    return val;
  }
  return null;
}

function parseArch(arch: unknown): string | null {
  if (typeof arch === 'string') {
    const val = arch.trim();
    if (val.length === 0) {
      return null;
    }
    return val;
  }
  return null;
}

function parsePlatform(value: unknown): Platform | null {
  if (typeof value !== 'string') {
    return null;
  }
  switch (value.trim().toLowerCase()) {
    case 'electron':
      return 'electron';
    case 'tauri':
      return 'tauri';
    default:
      return null;
  }
}

let userAgentInfo: UserAgentInfo | null = null;

function getUserAgentInfo(): UserAgentInfo {
  if (userAgentInfo != null) {
    return userAgentInfo;
  }
  if (typeof navigator === 'undefined' || typeof navigator?.userAgent !== 'string') {
    throw new Error(
      'Unable to retrieve User-Agent information. Please check if the environment you are running is a browser.'
    );
  }
  const userAgent = navigator.userAgent;
  const info = parseUserAgent(userAgent);
  if (info == null) {
    throw new Error(
      'Failed to parse User-Agent information. Please check if the `webview-bundle` is set to a current web environment.'
    );
  }
  userAgentInfo = info;
  return info;
}

export const env = {
  get version(): string {
    return getUserAgentInfo().version;
  },
  get platform(): Platform {
    return getUserAgentInfo().platform;
  },
  get platformVersion(): string {
    return getUserAgentInfo().platformVersion;
  },
  get isElectron(): boolean {
    return getUserAgentInfo().platform === 'electron';
  },
  get isTauri(): boolean {
    return getUserAgentInfo().platform === 'tauri';
  },
  get os(): string | undefined {
    return getUserAgentInfo().os;
  },
  get arch(): string | undefined {
    return getUserAgentInfo().arch;
  },
};
