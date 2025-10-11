import type { CloudFrontResponseEvent, CloudFrontResponseResult } from 'aws-lambda';

export function originResponseHandler() {
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
    const bundleName = response.headers['x-amz-meta-webview-bundle-name']?.[0]?.value;
    if (bundleName != null) {
      response.headers['webview-bundle-name'] = [{ key: 'Webview-Bundle-Name', value: bundleName }];
    }
    const version = response.headers['x-amz-meta-webview-bundle-version']?.[0]?.value;
    if (version != null) {
      response.headers['webview-bundle-version'] = [{ key: 'Webview-Bundle-Version', value: version }];
    }
    return response;
  };
}
