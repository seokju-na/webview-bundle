import { originRequest } from '@wvb/remote-aws/origin-request';

declare const __CONFIG__: any;

export const handler = originRequest(__CONFIG__);
