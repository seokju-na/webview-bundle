import { registerBindings, webviewBundle } from '@wvb/deno/desktop';

const wvb = webviewBundle({
  lib: new URL(`./.wvb/lib/${Deno.build.target}/`, import.meta.url),
  source: { appName: 'com.example.wvbapp' },
  routes: { '/': { bundle: 'app' } },
});

const window = new Deno.BrowserWindow({ title: 'Webview Bundle', width: 800, height: 600 });
registerBindings(window, wvb);
Deno.serve(wvb.fetch);
