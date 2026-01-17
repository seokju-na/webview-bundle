import { readBundle } from '@webview-bundle/node';
import { Command, Option } from 'clipanion';
import { cascade, isInExclusiveRange, isInteger, isNumber } from 'typanion';
import { resolveConfig } from '../config.js';
import { c, isColorEnabled } from '../console.js';
import { pathExists, toAbsolutePath } from '../fs.js';
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
    required: false,
  });
  readonly port = Option.String('--port,-P', '4312', {
    description: 'Specify a port number on which to start the http server. [Default: 4312] [env: PORT]',
    validator: cascade(isNumber(), [isInteger(), isInExclusiveRange(1, 65535)]),
    env: 'PORT',
  });
  readonly silent = Option.Boolean('--silent', {
    description: 'Disable log output.',
  });
  readonly configFile = Option.String('--config,-C', {
    description: 'Config file path',
  });
  readonly cwd = Option.String('--cwd', {
    description: 'Current working directory.',
  });

  async run() {
    const { Hono } = await import('hono');
    const { serve } = await import('@hono/node-server');

    const config = await resolveConfig({
      root: this.cwd,
      configFile: this.configFile,
    });
    const file = this.file ?? config.serve?.file;
    if (file == null) {
      this.logger.error(
        'Webview Bundle file is not specified. Set "serve.file" in the config file ' +
          'or pass [FILE] as a CLI argument.'
      );
      return 1;
    }
    const filepath = toAbsolutePath(file, config.root);
    if (!(await pathExists(filepath))) {
      this.logger.error(`File not found: ${filepath}`);
      return 1;
    }
    const bundle = await readBundle(filepath);
    const app = new Hono();

    const silent = this.silent ?? config.serve?.silent ?? false;
    if (!silent) {
      const { logMiddleware } = await import('../utils/hono-logger.js');
      app.use(
        logMiddleware(str => {
          this.logger.info(str);
        }, isColorEnabled())
      );
    }
    app.get('*', async c => {
      const p = this.resolvePath(c.req.path);
      if (!bundle.descriptor().index().containsPath(p)) {
        return c.notFound();
      }
      const entry = bundle.descriptor().index().getEntry(p)!;
      this.logger.debug(`Read entry: ${p} (content-type=${entry.contentType}, content-length=${entry.contentLength})`);
      const data = bundle.getData(p)!;
      for (const [name, value] of Object.entries(entry.headers)) {
        c.header(name, value, { append: true });
      }
      c.header('content-type', entry.contentType);
      c.header('content-length', String(entry.contentLength));
      return c.body(data as Uint8Array<ArrayBuffer>, 200);
    });
    const port = this.port ?? config.serve?.port ?? 4312;
    const server = serve({ fetch: app.fetch, port }, info => {
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
