import path from 'node:path';
import { serve } from '@hono/node-server';
import { readBundle } from '@webview-bundle/cli/binding';
import { Command, Option } from 'clipanion';
import { Hono } from 'hono';
import { logger } from 'hono/logger';
import { cascade, isInExclusiveRange, isInteger, isNumber } from 'typanion';
import { c } from '../console.js';
import { parseMimeType } from '../utils/mime-type.js';
import { BaseCommand } from './base.js';

export class ServeCommand extends BaseCommand {
  readonly name = 'serve';
  static paths = [['serve']];
  static usage = Command.Usage({
    description: 'Serve webview bundle files with localhost server.',
    examples: [
      ['A basic usage', '$0 serve ./dist.wvb'],
      ['Specify localhost port', '$0 serve ./dist.wvb --port 4312'],
    ],
  });

  readonly file = Option.String({
    name: 'FILE',
    required: true,
  });
  readonly port = Option.String('--port,-P', '4312', {
    validator: cascade(isNumber(), [isInteger(), isInExclusiveRange(1, 65535)]),
  });

  async run() {
    const filepath = path.isAbsolute(this.file) ? this.file : path.join(process.cwd(), this.file);
    const bundle = await readBundle(filepath);
    const app = new Hono();
    app.use(
      logger(str => {
        this.logger.info(str);
      })
    );
    app.get('*', async c => {
      const p = this.resolvePath(c.req.path);
      if (!bundle.manifest().index().containsPath(p)) {
        return c.notFound();
      }
      const entry = bundle.manifest().index().getEntry(p)!;
      const data = bundle.getData(p)!;
      for (const [name, value] of Object.entries(entry.headers)) {
        c.header(name, value, { append: true });
      }
      if (!c.res.headers.has('content-type')) {
        const mime = await parseMimeType(data, p);
        c.header('content-type', mime);
      }
      return c.body(data as Uint8Array<ArrayBuffer>, 200);
    });
    const server = serve({ fetch: app.fetch, port: this.port }, info => {
      this.logger.info(`Server started: ${c.success(`http://localhost:${info.port}`)}`);
    });
    const shutdown = () => {
      server.close(error => {
        if (error != null) {
          this.logger.error(`Server shutdown failed: {error}`, { error });
          process.exit(1);
        } else {
          process.exit(0);
        }
      });
    };
    process.on('SIGINT', shutdown);
    process.on('SIGTERM', shutdown);
  }

  private resolvePath(path: string) {
    if (path.endsWith('/')) {
      return `${path}index.html`;
    }
    const ext = path.split('.').pop();
    if (ext == null && !path.includes('.')) {
      return `${path}/index.html`;
    }
    return path;
  }
}
