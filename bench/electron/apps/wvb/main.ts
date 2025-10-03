import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { bundleProtocol, wvb } from '@webview-bundle/electron';
import { app, BrowserWindow } from 'electron';

const dirname = path.dirname(fileURLToPath(import.meta.url));

wvb({
  protocols: [bundleProtocol('app', path.join(dirname, '..', '..', '..', 'fixtures', 'next'))],
});

let mainWindow: BrowserWindow;

(async () => {
  await app.whenReady();
  mainWindow = new BrowserWindow();
  await mainWindow.loadURL('app://out.wvb');
})();
