import path from 'node:path';
import { readBundle } from '@webview-bundle/node';
import { Command, Option } from 'clipanion';
import { cascade, isInExclusiveRange, isInteger, isNumber } from 'typanion';
import { c } from '../console.js';
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
  readonly silent = Option.Boolean('--silent', false, {
    description: 'Disable log output.',
  });

  async run() {
    const { Hono } = await import('hono');
    const { serve } = await import('@hono/node-server');

    const filepath = path.isAbsolute(this.file) ? this.file : path.join(process.cwd(), this.file);
    const bundle = await readBundle(filepath);
    const app = new Hono();
    if (!this.silent) {
      // TODO: Need to rewrite it custom
      const { logger } = await import('hono/logger');
      app.use(
        logger(str => {
          this.logger.info(str);
        })
      );
    }
    app.get('*', async c => {
      const p = this.resolvePath(c.req.path);
      if (!bundle.descriptor().index().containsPath(p)) {
        return c.notFound();
      }
      const entry = bundle.descriptor().index().getEntry(p)!;
      const data = bundle.getData(p)!;
      for (const [name, value] of Object.entries(entry.headers)) {
        c.header(name, value, { append: true });
      }
      c.header('content-type', entry.contentType);
      c.header('content-length', String(entry.contentLength));
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
