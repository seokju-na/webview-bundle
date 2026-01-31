import { originRequest } from '@wvb/remote-aws-provider/origin-request';

declare const __CONFIG__: any;

export const handler = originRequest(__CONFIG__);
