import { registerBindings, webviewBundle } from '@wvb/deno/desktop';
import remote from './remote.json' with { type: 'json' };

const wvb = webviewBundle({
  lib: new URL(`./.wvb/lib/${Deno.build.target}/`, import.meta.url),
  source: { appName: 'dev.wvb.playground.deno' },
  updater: { remote },
  routes: { '/': { bundle: 'testbed' } },
});

const win = new Deno.BrowserWindow({
  title: 'Webview Bundle · Deno',
  width: 800,
  height: 600,
});
registerBindings(win, wvb);
Deno.serve(wvb.fetch);
