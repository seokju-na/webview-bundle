import fs from 'node:fs/promises';
import path from 'node:path';
import { type Bundle, decode } from '@webview-bundle/node-binding';

interface Config {
  bundleDir: string;
}

// 1. 번들 불러오는 로더 => 역할 분리 필요
// 2. 스킴 헬퍼 -> 커스텀 스킴을 적절하게 URL로 파싱해주고, 번들 내 파일 경로도 찾아줌
// 3. 번들에서 불러온 파일 데이터를 적절하게 Response 객체로 변환
// 4. 캐시?
export function protocolHandler({ bundleDir }: Config) {
  return async (request: Request): Promise<Response> => {
    const bundleFilepath = path.join(bundleDir, 'bundle.wvb');
    const bundle = await readBundle(bundleFilepath);

    // const url = new URL(request.url);
    // const filepath = resolveFilepath(url);
    // console.log(filepath);
    console.log(request.method, request.url);
    const filepath = request.url === 'app://test/' ? 'index.html' : 'index.js';
    const fileRaw = await bundle.readFile(filepath);
    const resp = new Response(fileRaw, {
      headers: {
        'content-type': filepath.endsWith('.html')
          ? 'text/html'
          : filepath.endsWith('.js')
            ? 'text/javascript'
            : 'text/plain',
      },
    });
    return resp;
  };
}

let bundle: Bundle | null = null;
async function readBundle(filepath: string): Promise<Bundle> {
  if (bundle != null) {
    return bundle;
  }
  const raw = await fs.readFile(filepath);
  bundle = await decode(raw);
  return bundle;
}

function resolveFilepath(url: URL): string {
  let filepath = stripStartSlash(url.pathname);
  if (path.extname(filepath) !== '') {
    return filepath;
  }
  filepath = filepath.endsWith('/') ? `${filepath}index.html` : `${filepath}/index.html`;
  return filepath;
}

function stripStartSlash(str: string): string {
  if (str.startsWith('/')) {
    return str.slice(1);
  }
  return str;
}
