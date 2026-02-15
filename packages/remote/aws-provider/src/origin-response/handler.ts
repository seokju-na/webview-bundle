import type {
  CloudFrontResponse,
  CloudFrontResponseEvent,
  CloudFrontResponseResult,
  Handler,
} from 'aws-lambda';
import { toAWSHeaderName } from '../utils.js';

export type OriginResponseHandler = Handler<CloudFrontResponseEvent, CloudFrontResponseResult>;

export function originResponse(): OriginResponseHandler {
  return async function handle(event: CloudFrontResponseEvent): Promise<CloudFrontResponseResult> {
    const request = event.Records[0]?.cf?.request;
    const response = event.Records[0]?.cf?.response;
    if (request == null || response == null) {
      throw new Error('invalid request');
    }
    const isWebviewRequest = request.headers['x-webview-bundle']?.[0] != null;
    if (!isWebviewRequest) {
      return response;
    }
    configureHeaders(response);
    return response;
  };
}

function configureHeaders(response: CloudFrontResponse): void {
  for (const [name, value] of Object.entries(response.headers)) {
    if (name.startsWith(`x-amz-meta-webview-bundle-`)) {
      const headerName = name.replace('x-amz-meta-', '');
      const headerValue = value[0]?.value;
      if (headerValue != null) {
        response.headers[headerName] = [
          {
            key: toAWSHeaderName(headerName),
            value: headerValue,
          },
        ];
      }
    }
  }
}
