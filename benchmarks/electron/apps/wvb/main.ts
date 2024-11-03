import path from 'node:path';
import { FSLoader, protocolHandler } from '@webview-bundle/electron';
import { BrowserWindow, app, protocol } from 'electron';

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
  loader: FSLoader.fromDir(path.join(__dirname, '../../../fixtures/next')),
  cache: new Map(),
});

let mainWindow: BrowserWindow;

(async () => {
  await app.whenReady();
  protocol.handle('app', handler);
  mainWindow = new BrowserWindow();
  await mainWindow.loadURL('app://out.wvb');
})();
