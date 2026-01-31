import { wvbRemote } from '@wvb/remote-cloudflare-provider';

const remote = wvbRemote();

export default {
  async fetch(req: Request, env: any): Promise<Response> {
    return remote.fetch(req, { kv: env.KV, r2: env.BUCKET });
  },
};
