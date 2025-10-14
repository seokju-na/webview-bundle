import { originRequest } from '@webview-bundle/remote-aws/origin-request';

declare const __CONFIG__: any;

export const handler = originRequest(__CONFIG__);
