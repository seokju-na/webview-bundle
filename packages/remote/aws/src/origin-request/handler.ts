import type { CloudFrontRequestEvent, CloudFrontRequestResult, Handler } from 'aws-lambda';
import { handle } from 'hono/lambda-edge';
import { type WebviewBundleRemoteConfig, wvbRemote } from '../remote.js';

export type OriginRequestHandler = Handler<CloudFrontRequestEvent, CloudFrontRequestResult>;

export function originRequest(config: WebviewBundleRemoteConfig): OriginRequestHandler {
  return handle(wvbRemote(config)) as OriginRequestHandler;
}
