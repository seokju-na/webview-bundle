import { type WebviewBundleRemoteConfig, wvbRemote } from '@webview-bundle/remote-aws';
import { handle } from 'hono/lambda-edge';

declare const __CONFIG__: WebviewBundleRemoteConfig;

export const handler: any = handle(wvbRemote(__CONFIG__));
