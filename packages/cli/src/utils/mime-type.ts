import { fileTypeFromBuffer } from 'file-type';

/**
 * Web Compatible MimeTypes
 * @see https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types#important_mime_types_for_web_developers
 */
export type MimeType =
  | 'text/css'
  | 'text/csv'
  | 'text/html'
  | 'image/vnd.microsoft.icon'
  | 'text/javascript'
  | 'application/json'
  | 'application/ld+json'
  | 'video/mp4'
  | 'application/octet-stream'
  | 'application/rtf'
  | 'image/svg+xml'
  | 'text/plain';

export function parseMimeTypeFromUri(uri: string): MimeType | undefined {
  const suffix = uri.split('.').pop();
  if (suffix === uri) {
    return 'application/octet-stream';
  }
  switch (suffix) {
    case 'css':
    case 'less':
    case 'sass':
    case 'styl':
      return 'text/css';
    case 'csv':
      return 'text/csv';
    case 'html':
      return 'text/html';
    case 'ico':
      return 'image/vnd.microsoft.icon';
    case 'js':
    case 'mjs':
      return 'text/javascript';
    case 'json':
      return 'application/json';
    case 'jsonld':
      return 'application/ld+json';
    case 'mp4':
      return 'video/mp4';
    case 'rtf':
      return 'application/rtf';
    case 'svg':
      return 'image/svg+xml';
    case 'txt':
      return 'text/plain';
    case 'bin':
      return 'application/octet-stream';
  }
  return undefined;
}

export async function parseMimeType(content: Uint8Array, uri: string): Promise<MimeType> {
  const inferred = uri.endsWith('.svg') ? undefined : await fileTypeFromBuffer(content);
  if (inferred != null) {
    return inferred.mime as MimeType;
  }
  return parseMimeTypeFromUri(uri) ?? 'text/html';
}
