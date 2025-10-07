import { wvbRemote } from '../src/index.js';

const remote = wvbRemote();

export default {
  async fetch(req: Request, env: Env): Promise<Response> {
    return await remote.fetch(req, { kv: env.TEST_KV, r2: env.TEST_BUCKET });
  },
};
