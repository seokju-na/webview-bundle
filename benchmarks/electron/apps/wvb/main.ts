import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { FSLoader, protocolHandler } from '@webview-bundle/electron';
import { app, BrowserWindow, protocol } from 'electron';

const dirname = path.dirname(fileURLToPath(import.meta.url));

protocol.registerSchemesAsPrivileged([
  {
    scheme: 'app',
    privileges: {
      standard: true,
      secure: true,
      allowServiceWorkers: true,
      supportFetchAPI: true,
      corsEnabled: true,
    },
  },
]);

const handler = protocolHandler({
  loader: FSLoader.fromDir(path.join(dirname, '..', '..', '..', 'fixtures', 'next')),
  cache: new Map(),
});

let mainWindow: BrowserWindow;

(async () => {
  await app.whenReady();
  protocol.handle('app', handler);
  mainWindow = new BrowserWindow();
  await mainWindow.loadURL('app://out.wvb');
})();
